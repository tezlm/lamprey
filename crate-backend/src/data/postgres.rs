use sqlx::PgPool;

use super::Data;

#[derive(Debug)]
pub struct Postgres {
    pub(crate) pool: PgPool,
    // TODO: make postgres use one transaction + smaller queries?
    // pub(crate) conn: PgConnection,
}

impl Data for Postgres {}

mod application;
mod audit_logs;
mod auth;

mod embed;
mod emoji;
mod invite;
mod media;
mod message;
mod permission;
mod reaction;
mod role;
mod role_member;
mod room;
mod room_member;
mod search;
mod session;
mod thread;
mod thread_member;
mod unread;
mod user;
mod user_config;
mod user_relationship;
mod util;

pub use util::Pagination;
