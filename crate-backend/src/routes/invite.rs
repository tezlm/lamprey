use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::{extract::State, Json};
use common::v1::types::{
    Invite, InviteCode, InviteCreate, InvitePatch, InviteTarget, InviteTargetId,
    InviteWithMetadata, MessageSync, PaginationQuery, PaginationResponse, Permission, RoomId,
    RoomMembership,
};
use futures::future::join_all;
use http::StatusCode;
use nanoid::nanoid;
use serde::Serialize;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::error::Result;
use crate::{Error, ServerState};

use super::util::{Auth, HeaderReason};

/// Invite delete
#[utoipa::path(
    delete,
    path = "/invite/{invite_code}",
    params(
        ("invite_code", description = "The code identifying this invite"),
    ),
    tags = ["invite"],
    responses(
        (status = NO_CONTENT, description = "success"),
    )
)]
pub async fn invite_delete(
    Path(code): Path<InviteCode>,
    Auth(user_id): Auth,
    HeaderReason(reason): HeaderReason,
    State(s): State<Arc<ServerState>>,
) -> Result<impl IntoResponse> {
    let d = s.data();
    let invite = d.invite_select(code.clone()).await?;
    let (has_perm, id_target) = match invite.invite.target {
        InviteTarget::Room { room } => (
            s.services()
                .perms
                .for_room(user_id, room.id)
                .await?
                .has(Permission::InviteManage),
            InviteTargetId::Room { room_id: room.id },
        ),
        InviteTarget::Thread { room, thread } => (
            s.services()
                .perms
                .for_thread(user_id, thread.id)
                .await?
                .has(Permission::InviteManage),
            InviteTargetId::Thread {
                room_id: room.id,
                thread_id: thread.id,
            },
        ),
        InviteTarget::Server => (
            s.services.perms.is_admin(user_id).await?,
            InviteTargetId::Server,
        ),
    };
    let can_delete = user_id == invite.invite.creator_id || has_perm;
    if can_delete {
        d.invite_delete(code.clone()).await?;
        match id_target {
            InviteTargetId::Room { room_id } => {
                s.broadcast_room(
                    room_id,
                    user_id,
                    reason,
                    MessageSync::InviteDelete {
                        code,
                        target: id_target,
                    },
                )
                .await?;
            }
            InviteTargetId::Thread { thread_id, .. } => {
                s.broadcast_thread(
                    thread_id,
                    user_id,
                    reason,
                    MessageSync::InviteDelete {
                        code,
                        target: id_target,
                    },
                )
                .await?;
            }
            InviteTargetId::Server => {
                // don't emit events
            }
        };
    }
    Ok(StatusCode::NO_CONTENT)
}

/// Invite resolve
#[utoipa::path(
    get,
    path = "/invite/{invite_code}",
    params(
        ("invite_code", description = "The code identifying this invite"),
    ),
    tags = ["invite"],
    responses(
        (status = OK, body = Invite, description = "success"),
        (status = OK, body = InviteWithMetadata, description = "success with metadata"),
    )
)]
pub async fn invite_resolve(
    Path(code): Path<InviteCode>,
    Auth(user_id): Auth,
    State(s): State<Arc<ServerState>>,
) -> Result<impl IntoResponse> {
    let d = s.data();
    let s = s.services();
    let invite = d.invite_select(code).await?;
    if invite.invite.creator_id == user_id {
        return Ok(Json(invite).into_response());
    }
    let should_strip = match &invite.invite.target {
        InviteTarget::Room { room } => {
            let perms = s.perms.for_room(user_id, room.id).await?;
            !perms.has(Permission::InviteManage)
        }
        InviteTarget::Thread { room: _, thread } => {
            let perms = s.perms.for_thread(user_id, thread.id).await?;
            !perms.has(Permission::InviteManage)
        }
        InviteTarget::Server => !s.perms.is_admin(user_id).await?,
    };
    if should_strip {
        Ok(Json(invite.strip_metadata()).into_response())
    } else {
        Ok(Json(invite).into_response())
    }
}

/// Invite use
///
/// - A room invite will add the user to the room
/// - A thread invite will currently do the same thing as a room invite
/// - A server invite will upgrade the user to a full account
#[utoipa::path(
    post,
    path = "/invite/{invite_code}",
    params(
        ("invite_code", description = "The code identifying this invite"),
    ),
    tags = ["invite"],
    responses(
        (status = OK, description = "success"),
    )
)]
pub async fn invite_use(
    Path(code): Path<InviteCode>,
    Auth(user_id): Auth,
    HeaderReason(reason): HeaderReason,
    State(s): State<Arc<ServerState>>,
) -> Result<impl IntoResponse> {
    let d = s.data();
    let invite = d.invite_select(code.clone()).await?;
    if invite.is_dead() {
        return Err(Error::NotFound);
    }
    match invite.invite.target {
        InviteTarget::Thread { room, .. } | InviteTarget::Room { room } => {
            // TODO: any thread-specific invite things?
            d.room_member_put(
                room.id,
                user_id,
                RoomMembership::Join {
                    override_name: None,
                    override_description: None,
                    roles: vec![],
                },
            )
            .await?;
            d.role_apply_default(room.id, user_id).await?;
            let member = d.room_member_get(room.id, user_id).await?;
            s.services.perms.invalidate_room(user_id, room.id).await;
            s.services.perms.invalidate_is_mutual(user_id);
            s.broadcast_room(
                room.id,
                user_id,
                reason,
                MessageSync::RoomMemberUpsert { member },
            )
            .await?;
        }
        InviteTarget::Server => {
            // Upgrade the user to a full account
            s.data()
                .user_set_registered_at(user_id, Some(common::v1::types::util::Time::now_utc()))
                .await?;
            s.services().users.invalidate(user_id).await;
            let updated_user = s.services().users.get(user_id).await?;
            s.broadcast(MessageSync::UserUpdate { user: updated_user })?;
        }
    }
    d.invite_incr_use(code).await?;
    Ok(())
}

/// Invite room create
///
/// Create an invite that goes to a room
#[utoipa::path(
    post,
    path = "/room/{room_id}/invite",
    params(
        ("room_id", description = "Room id"),
    ),
    tags = ["invite"],
    responses(
        (status = OK, body = Invite, description = "success"),
    )
)]
pub async fn invite_room_create(
    Path(room_id): Path<RoomId>,
    Auth(user_id): Auth,
    HeaderReason(reason): HeaderReason,
    State(s): State<Arc<ServerState>>,
    Json(json): Json<InviteCreate>,
) -> Result<impl IntoResponse> {
    let d = s.data();
    let perms = s.services.perms.for_room(user_id, room_id).await?;
    perms.ensure_view()?;
    perms.ensure(Permission::InviteCreate)?;
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let code = InviteCode(nanoid!(8, &alphabet));
    d.invite_insert_room(
        room_id,
        user_id,
        code.clone(),
        json.expires_at,
        json.max_uses,
    )
    .await?;
    let invite = d.invite_select(code).await?;
    s.broadcast_room(
        room_id,
        user_id,
        reason,
        MessageSync::InviteCreate {
            invite: invite.clone(),
        },
    )
    .await?;
    Ok((StatusCode::CREATED, Json(invite)))
}

#[derive(Serialize)]
#[serde(untagged)]
enum InviteWithPotentialMetadata {
    Invite(Invite),
    InviteWithMetadata(InviteWithMetadata),
}

/// Invite room list
///
/// List invites that go to a room
#[utoipa::path(
    get,
    path = "/room/{room_id}/invite",
    params(
        PaginationQuery<InviteCode>,
        ("room_id", description = "Room id"),
    ),
    tags = ["invite"],
    responses(
        (status = OK, body = PaginationResponse<Invite>, description = "success"),
    )
)]
pub async fn invite_room_list(
    Path(room_id): Path<RoomId>,
    Query(paginate): Query<PaginationQuery<InviteCode>>,
    Auth(user_id): Auth,
    State(s): State<Arc<ServerState>>,
) -> Result<impl IntoResponse> {
    let d = s.data();
    let perms = s.services.perms.for_room(user_id, room_id).await?;
    perms.ensure_view()?;
    let res = d.invite_list_room(room_id, paginate).await?;
    let res = PaginationResponse {
        items: res
            .items
            .into_iter()
            .map(|i| {
                if i.invite.creator_id != user_id && !perms.has(Permission::InviteManage) {
                    InviteWithPotentialMetadata::Invite(i.strip_metadata())
                } else {
                    InviteWithPotentialMetadata::InviteWithMetadata(i)
                }
            })
            .collect(),
        total: res.total,
        has_more: res.has_more,
    };
    Ok(Json(res))
}

/// Invite patch (TODO)
///
/// Edit an invite
#[utoipa::path(
    patch,
    path = "/invite/{invite_code}",
    params(
        ("invite_code", description = "The code identifying this invite"),
    ),
    tags = ["invite"],
    responses(
        (status = NOT_MODIFIED, description = "not modified"),
        (status = OK, body = Invite, description = "success"),
    )
)]
pub async fn invite_patch(
    Path(code): Path<InviteCode>,
    Auth(user_id): Auth,
    HeaderReason(_reason): HeaderReason,
    State(s): State<Arc<ServerState>>,
    Json(patch): Json<InvitePatch>,
) -> Result<impl IntoResponse> {
    let d = s.data();
    let invite = d.invite_select(code.clone()).await?;

    let (has_perm, _id_target) = match invite.invite.target {
        InviteTarget::Room { room } => (
            s.services()
                .perms
                .for_room(user_id, room.id)
                .await?
                .has(Permission::InviteManage),
            InviteTargetId::Room { room_id: room.id },
        ),
        InviteTarget::Thread { room, thread } => (
            s.services()
                .perms
                .for_thread(user_id, thread.id)
                .await?
                .has(Permission::InviteManage),
            InviteTargetId::Thread {
                room_id: room.id,
                thread_id: thread.id,
            },
        ),
        InviteTarget::Server => (
            s.services.perms.is_admin(user_id).await?,
            InviteTargetId::Server,
        ),
    };

    let can_patch = user_id == invite.invite.creator_id || has_perm;
    if !can_patch {
        return Err(Error::MissingPermissions);
    }

    let updated_invite = d.invite_update(code.clone(), patch).await?;

    // TODO: Check if any actual changes were made and return NOT_MODIFIED if not.
    // This would require comparing `invite` and `updated_invite`.

    s.broadcast(MessageSync::InviteUpdate {
        invite: updated_invite.clone(),
    })?;

    Ok((StatusCode::OK, Json(updated_invite)))
}

/// Invite server create
///
/// Create an invite that allows registration on a server.
#[utoipa::path(
    post,
    path = "/server/invite",
    tags = ["invite"],
    responses((status = OK, body = Invite, description = "success")),
)]
pub async fn invite_server_create(
    Auth(user_id): Auth,
    State(s): State<Arc<ServerState>>,
    HeaderReason(reason): HeaderReason,
    Json(json): Json<InviteCreate>,
) -> Result<impl IntoResponse> {
    let d = s.data();
    let user = s.services().users.get(user_id).await?;
    if user.registered_at.is_none() {
        return Err(Error::BadStatic("Guest users cannot create server invites"));
    }

    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let code = InviteCode(nanoid!(8, &alphabet));
    d.invite_insert_server(user_id, code.clone(), json.expires_at, json.max_uses)
        .await?;
    let invite = d.invite_select(code).await?;
    s.broadcast(MessageSync::InviteCreate {
        invite: invite.clone(),
    })?;
    Ok((StatusCode::CREATED, Json(invite)))
}

/// Invite server list
///
/// List invites that allow registration on a server
#[utoipa::path(
    get,
    path = "/server/invite",
    params(
        PaginationQuery<InviteCode>,
    ),
    tags = ["invite"],
    responses(
        (status = OK, body = PaginationResponse<Invite>, description = "success"),
    )
)]
pub async fn invite_server_list(
    Query(paginate): Query<PaginationQuery<InviteCode>>,
    Auth(user_id): Auth,
    State(s): State<Arc<ServerState>>,
) -> Result<impl IntoResponse> {
    let d = s.data();
    let user = s.services().users.get(user_id).await?;
    if user.registered_at.is_none() {
        return Err(Error::BadStatic("Guest users cannot list server invites"));
    }
    let res = d.invite_list_server(paginate).await?;
    let items: Vec<InviteWithPotentialMetadata> = join_all(res.items.into_iter().map(|i| {
        let s = s.clone();
        async move {
            if i.invite.creator_id != user_id
                && !s.services.perms.is_admin(user_id).await.unwrap_or(false)
            {
                InviteWithPotentialMetadata::Invite(i.strip_metadata())
            } else {
                InviteWithPotentialMetadata::InviteWithMetadata(i)
            }
        }
    }))
    .await;
    let res = PaginationResponse {
        items,
        total: res.total,
        has_more: res.has_more,
    };
    Ok(Json(res))
}

pub fn routes() -> OpenApiRouter<Arc<ServerState>> {
    OpenApiRouter::new()
        .routes(routes!(invite_delete))
        .routes(routes!(invite_resolve))
        .routes(routes!(invite_patch))
        .routes(routes!(invite_use))
        .routes(routes!(invite_room_create))
        .routes(routes!(invite_room_list))
        .routes(routes!(invite_server_create))
        .routes(routes!(invite_server_list))
}
