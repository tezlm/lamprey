use std::sync::Arc;

use crate::routes::util::HeaderReason;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use common::v1::types::{MessageSync, Permission, PermissionOverwrite, PermissionOverwriteSet};
use common::v1::types::{PermissionOverwriteType, ThreadId};
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use super::util::Auth;
use crate::error::{Error, Result};
use crate::ServerState;

/// Thread permission overwrite
#[utoipa::path(
    put,
    path = "/thread/{thread_id}/permission/{overwrite_id}",
    params(
        ("thread_id", description = "Thread id"),
        ("overwrite_id", description = "Role or user id"),
    ),
    tags = ["thread"],
    responses((status = NO_CONTENT, description = "success"))
)]
async fn permission_thread_overwrite(
    Auth(auth_user_id): Auth,
    State(s): State<Arc<ServerState>>,
    Path((thread_id, overwrite_id)): Path<(ThreadId, Uuid)>,
    HeaderReason(reason): HeaderReason,
    Json(json): Json<PermissionOverwriteSet>,
) -> Result<impl IntoResponse> {
    let perms = s
        .services()
        .perms
        .for_thread(auth_user_id, thread_id)
        .await?;
    perms.ensure_view()?;
    perms.ensure(Permission::RoleManage)?;
    // FIXME: allow editing permissions you can edit as long as you don't edit ones you can't
    // word salad sandwich
    for p in &json.allow {
        perms.ensure(*p)?;
    }
    for p in &json.deny {
        perms.ensure(*p)?;
    }
    s.services()
        .perms
        .permission_overwrite_upsert(
            thread_id,
            overwrite_id,
            PermissionOverwriteType::User,
            json.allow,
            json.deny,
        )
        .await?;

    let srv = s.services();
    let thread = srv.threads.get(thread_id, Some(auth_user_id)).await?;
    s.broadcast_thread(
        thread_id,
        auth_user_id,
        reason,
        MessageSync::ThreadUpdate { thread },
    )
    .await?;
    Ok(())
}

/// Thread permission delete
#[utoipa::path(
    delete,
    path = "/thread/{thread_id}/permission/{overwrite_id}",
    params(
        ("thread_id", description = "Thread id"),
        ("overwrite_id", description = "Role or user id"),
    ),
    tags = ["thread"],
    responses((status = NO_CONTENT, description = "success"))
)]
async fn permission_thread_delete(
    Auth(auth_user_id): Auth,
    State(s): State<Arc<ServerState>>,
    Path((thread_id, overwrite_id)): Path<(ThreadId, Uuid)>,
    HeaderReason(reason): HeaderReason,
) -> Result<Json<()>> {
    let perms = s
        .services()
        .perms
        .for_thread(auth_user_id, thread_id)
        .await?;
    perms.ensure_view()?;
    perms.ensure(Permission::RoleManage)?;
    s.services()
        .perms
        .permission_overwrite_delete(thread_id, overwrite_id)
        .await?;
    let srv = s.services();
    let thread = srv.threads.get(thread_id, Some(auth_user_id)).await?;
    s.broadcast_thread(
        thread_id,
        auth_user_id,
        reason,
        MessageSync::ThreadUpdate { thread },
    )
    .await?;
    Ok(Json(()))
}

/// Tag permission override upsert (TODO)
///
/// Upsert a tag permission override
#[utoipa::path(
    put,
    path = "/room/{room_id}/tag/{tag_id}/permission/{overwrite_id}",
    params(
        ("room_id", description = "Room id"),
        ("tag_id", description = "Tag id"),
        ("overwrite_id", description = "Role or user id"),
    ),
    tags = ["tag"],
    responses((status = OK, body = PermissionOverwrite, description = "success"))
)]
async fn permission_tag_overwrite(
    Auth(_auth_user_id): Auth,
    State(_s): State<Arc<ServerState>>,
    Json(_json): Json<PermissionOverwrite>,
) -> Result<Json<PermissionOverwrite>> {
    Err(Error::Unimplemented)
}

/// Tag permission override delete (TODO)
///
/// Delete a tag permission override
#[utoipa::path(
    delete,
    path = "/room/{room_id}/tag/{tag_id}/permission/{overwrite_id}",
    params(
        ("room_id", description = "Room id"),
        ("tag_id", description = "Tag id"),
        ("overwrite_id", description = "Role or user id"),
    ),
    tags = ["tag"],
    responses((status = NO_CONTENT, description = "success"))
)]
async fn permission_tag_delete(
    Auth(_auth_user_id): Auth,
    State(_s): State<Arc<ServerState>>,
) -> Result<Json<()>> {
    Err(Error::Unimplemented)
}

pub fn routes() -> OpenApiRouter<Arc<ServerState>> {
    OpenApiRouter::new()
        .routes(routes!(permission_thread_overwrite))
        .routes(routes!(permission_thread_delete))
        .routes(routes!(permission_tag_overwrite))
        .routes(routes!(permission_tag_delete))
}
