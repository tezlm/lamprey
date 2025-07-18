use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use common::v1::types::{
    application::{Application, ApplicationCreate},
    util::Time,
    ApplicationId, Bot, BotAccess, ExternalPlatform, PaginationQuery, Puppet, PuppetCreate,
    SessionCreate, SessionStatus, SessionToken, SessionWithToken,
};
use http::StatusCode;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;
use validator::Validate;

use crate::{types::DbUserCreate, ServerState};

use super::util::Auth;
use crate::error::{Error, Result};

/// App create
#[utoipa::path(
    post,
    path = "/app",
    tags = ["application"],
    responses((status = CREATED, description = "success"))
)]
async fn app_create(
    Auth(auth_user_id): Auth,
    State(s): State<Arc<ServerState>>,
    Json(json): Json<ApplicationCreate>,
) -> Result<impl IntoResponse> {
    let data = s.data();
    let user = data
        .user_create(DbUserCreate {
            parent_id: Some(auth_user_id),
            name: json.name.clone(),
            description: json.description.clone(),
            bot: Some(Bot {
                owner_id: auth_user_id,
                access: if json.public {
                    return Err(Error::Unimplemented);
                } else {
                    BotAccess::Private
                },
                is_bridge: json.bridge,
            }),
            puppet: None,
            registered_at: Some(Time::now_utc()),
        })
        .await?;
    let app = Application {
        id: user.id.into_inner().into(),
        owner_id: auth_user_id,
        name: json.name,
        description: json.description,
        bridge: json.bridge,
        public: json.public,
    };
    data.application_insert(app.clone()).await?;
    Ok(Json(app))
}

/// App list
#[utoipa::path(
    get,
    path = "/app",
    tags = ["application"],
    responses((status = OK, description = "success"))
)]
async fn app_list(
    Auth(auth_user_id): Auth,
    Query(q): Query<PaginationQuery<ApplicationId>>,
    State(s): State<Arc<ServerState>>,
) -> Result<impl IntoResponse> {
    let data = s.data();
    let list = data.application_list(auth_user_id, q).await?;
    Ok(Json(list))
}

/// App get
#[utoipa::path(
    get,
    path = "/app/{app_id}",
    tags = ["application"],
    responses((status = OK, description = "success"))
)]
async fn app_get(
    Path((app_id,)): Path<(ApplicationId,)>,
    Auth(auth_user_id): Auth,
    State(s): State<Arc<ServerState>>,
) -> Result<impl IntoResponse> {
    let data = s.data();
    let app = data.application_get(app_id).await?;
    if app.owner_id == auth_user_id {
        Ok(Json(app))
    } else {
        Err(Error::NotFound)
    }
}

/// App patch
#[utoipa::path(
    patch,
    path = "/app/{app_id}",
    tags = ["application"],
    responses((status = OK, description = "success"))
)]
async fn app_patch(Auth(_auth_user_id): Auth, State(_s): State<Arc<ServerState>>) -> Result<()> {
    Err(Error::Unimplemented)
}

/// App delete
#[utoipa::path(
    delete,
    path = "/app/{app_id}",
    tags = ["application"],
    responses((status = OK, description = "success"))
)]
async fn app_delete(
    Path((app_id,)): Path<(ApplicationId,)>,
    Auth(auth_user_id): Auth,
    State(s): State<Arc<ServerState>>,
) -> Result<impl IntoResponse> {
    let data = s.data();
    let app = data.application_get(app_id).await?;
    if app.owner_id == auth_user_id {
        data.application_delete(app_id).await?;
        data.user_delete(app_id.into_inner().into()).await?;
    }
    Ok(StatusCode::NO_CONTENT)
}

/// App create session
#[utoipa::path(
    post,
    path = "/app/{app_id}/session",
    tags = ["application"],
    responses((status = OK, description = "success"))
)]
async fn app_create_session(
    Path((app_id,)): Path<(ApplicationId,)>,
    Auth(auth_user_id): Auth,
    State(s): State<Arc<ServerState>>,
    Json(json): Json<SessionCreate>,
) -> Result<impl IntoResponse> {
    json.validate()?;
    let data = s.data();
    let app = data.application_get(app_id).await?;
    if app.owner_id == auth_user_id {
        let token = SessionToken(Uuid::new_v4().to_string()); // TODO: is this secure enough
        let session = data.session_create(token.clone(), json.name).await?;
        data.session_set_status(
            session.id,
            SessionStatus::Authorized {
                user_id: app.id.into_inner().into(),
            },
        )
        .await?;
        let session = data.session_get(session.id).await?;
        let session_with_token = SessionWithToken { session, token };
        Ok((StatusCode::CREATED, Json(session_with_token)))
    } else {
        Err(Error::MissingPermissions)
    }
}

/// Puppet ensure
#[utoipa::path(
    put,
    path = "/app/{app_id}/puppet/{puppet_id}",
    tags = ["application"],
    responses((status = OK, description = "success"))
)]
async fn puppet_ensure(
    Path((app_id, puppet_id)): Path<(ApplicationId, String)>,
    Auth(auth_user_id): Auth,
    State(s): State<Arc<ServerState>>,
    Json(json): Json<PuppetCreate>,
) -> Result<impl IntoResponse> {
    if *app_id != *auth_user_id {
        return Err(Error::MissingPermissions);
    }

    let parent_id = Some(auth_user_id);
    let data = s.data();
    let srv = s.services();
    let parent = srv.users.get(auth_user_id).await?;
    if !parent.bot.is_some_and(|b| b.is_bridge) {
        return Err(Error::BadStatic("can't create that user"));
    };
    let existing = data
        .user_lookup_puppet(dbg!(auth_user_id), dbg!(&puppet_id))
        .await?;
    if let Some(id) = dbg!(existing) {
        let user = data.user_get(id).await?;
        return Ok((StatusCode::OK, Json(user)));
    }
    let user = data
        .user_create(DbUserCreate {
            parent_id,
            name: json.name,
            description: json.description,
            bot: None,
            puppet: Some(Puppet {
                owner_id: auth_user_id,
                external_platform: ExternalPlatform::Discord,
                external_id: puppet_id.clone(),
                external_url: None,
                alias_id: None,
            }),
            registered_at: Some(Time::now_utc()),
        })
        .await?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub fn routes() -> OpenApiRouter<Arc<ServerState>> {
    OpenApiRouter::new()
        .routes(routes!(app_create))
        .routes(routes!(app_list))
        .routes(routes!(app_get))
        .routes(routes!(app_patch))
        .routes(routes!(app_delete))
        .routes(routes!(app_create_session))
        .routes(routes!(puppet_ensure))
}
