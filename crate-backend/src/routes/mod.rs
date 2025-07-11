use std::sync::Arc;

use utoipa_axum::router::OpenApiRouter;

use crate::ServerState;

mod application;
mod auth;
mod debug;

mod emoji;
mod internal;
mod invite;
mod media;
mod message;
mod moderation;
mod notification;
mod permission_overwrite;
mod reaction;
mod relationship;
mod role;
mod room;
mod room_member;
mod search;
mod session;
mod sync;
mod tag;
mod thread;
mod thread_member;
mod user;
mod user_config;
mod user_email;
mod util;
mod voice;

pub fn routes() -> OpenApiRouter<Arc<ServerState>> {
    OpenApiRouter::new()
        .merge(application::routes())
        .merge(auth::routes())
        .merge(debug::routes())
        .merge(emoji::routes())
        .merge(internal::routes())
        .merge(invite::routes())
        .merge(media::routes())
        .merge(message::routes())
        .merge(moderation::routes())
        .merge(notification::routes())
        .merge(permission_overwrite::routes())
        .merge(reaction::routes())
        .merge(relationship::routes())
        .merge(role::routes())
        .merge(room::routes())
        .merge(room_member::routes())
        .merge(search::routes())
        .merge(session::routes())
        .merge(sync::routes())
        .merge(tag::routes())
        .merge(thread::routes())
        .merge(thread_member::routes())
        .merge(user::routes())
        .merge(user_config::routes())
        .merge(user_email::routes())
        .merge(voice::routes())
}
