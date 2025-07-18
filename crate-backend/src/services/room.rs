use std::sync::Arc;

use common::v1::types::defaults::{EVERYONE_TRUSTED, MODERATOR};
use common::v1::types::util::Diff;
use common::v1::types::{Permission, Room, RoomCreate, RoomId, RoomMembership, RoomPatch, UserId};
use moka::future::Cache;

use crate::error::{Error, Result};
use crate::types::DbRoleCreate;
use crate::ServerStateInner;

pub struct ServiceRooms {
    state: Arc<ServerStateInner>,
    cache_room: Cache<RoomId, Room>,
}

impl ServiceRooms {
    pub fn new(state: Arc<ServerStateInner>) -> Self {
        Self {
            state,
            cache_room: Cache::builder()
                .max_capacity(100_000)
                .support_invalidation_closures()
                .build(),
        }
    }

    pub async fn get(&self, room_id: RoomId, _user_id: Option<UserId>) -> Result<Room> {
        self.cache_room
            .try_get_with(room_id, self.state.data().room_get(room_id))
            .await
            .map_err(|err| err.fake_clone())
    }

    pub async fn update(&self, room_id: RoomId, user_id: UserId, patch: RoomPatch) -> Result<Room> {
        let data = self.state.data();
        let start = data.room_get(room_id).await?;
        if !patch.changes(&start) {
            return Err(Error::NotModified);
        }

        data.room_update(room_id, patch).await?;
        self.cache_room
            .remove(&room_id)
            .await
            .expect("failed to invalidate");
        self.get(room_id, Some(user_id)).await
    }

    pub async fn create(&self, create: RoomCreate, creator: UserId) -> Result<Room> {
        let data = self.state.data();
        let room = data.room_create(create).await?;
        let room_id = room.id;
        let role_admin = DbRoleCreate {
            room_id,
            name: "admin".to_owned(),
            description: None,
            permissions: vec![Permission::Admin],
            is_self_applicable: false,
            is_mentionable: false,
            is_default: false,
        };
        let role_moderator = DbRoleCreate {
            room_id,
            name: "moderator".to_owned(),
            description: None,
            permissions: MODERATOR.to_vec(),
            is_self_applicable: false,
            is_mentionable: false,
            is_default: false,
        };
        let role_everyone = DbRoleCreate {
            room_id,
            name: "everyone".to_owned(),
            description: None,
            permissions: EVERYONE_TRUSTED.to_vec(),
            is_self_applicable: false,
            is_mentionable: false,
            is_default: true,
        };
        let admin = data.role_create(role_admin).await?;
        data.role_create(role_moderator).await?;
        data.role_create(role_everyone).await?;
        data.room_member_put(
            room_id,
            creator,
            RoomMembership::Join {
                override_name: None,
                override_description: None,
                roles: vec![],
            },
        )
        .await?;
        data.role_member_put(creator, admin.id).await?;
        data.role_apply_default(room.id, creator).await?;
        Ok(room)
    }

    pub async fn create_dm(&self, user_a_id: UserId, user_b_id: UserId) -> Result<Room> {
        let data = self.state.data();
        let room = data
            .room_create(RoomCreate {
                name: "(dm)".into(),
                description: None,
                icon: None,
            })
            .await?;
        let room_id = room.id;
        let role_default = DbRoleCreate {
            room_id,
            name: "default".to_owned(),
            description: None,
            permissions: vec![Permission::Admin],
            is_self_applicable: false,
            is_mentionable: false,
            is_default: true,
        };
        data.role_create(role_default).await?;
        data.room_member_put(
            room_id,
            user_a_id,
            RoomMembership::Join {
                override_name: None,
                override_description: None,
                roles: vec![],
            },
        )
        .await?;
        data.room_member_put(
            room_id,
            user_b_id,
            RoomMembership::Join {
                override_name: None,
                override_description: None,
                roles: vec![],
            },
        )
        .await?;
        data.role_apply_default(room.id, user_a_id).await?;
        data.role_apply_default(room.id, user_b_id).await?;
        Ok(room)
    }
}
