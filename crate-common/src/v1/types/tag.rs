use serde::{Deserialize, Serialize};

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

#[cfg(feature = "validator")]
use validator::Validate;

use crate::v1::types::{misc::Color, util::some_option, Room, RoomId, TagId, TagVerId, Thread};

// hmm, should i be able to apply tags to other tags?
// tagception!

/// a tag that can be applied to things
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "validator", derive(Validate))]
pub struct Tag {
    pub id: TagId,
    pub version_id: TagVerId,
    pub room_id: RoomId,

    #[cfg_attr(feature = "utoipa", schema(min_length = 1, max_length = 64))]
    #[cfg_attr(feature = "validator", validate(length(min = 1, max = 64)))]
    pub name: String,

    #[cfg_attr(feature = "utoipa", schema(min_length = 1, max_length = 8192))]
    #[cfg_attr(feature = "validator", validate(length(min = 1, max = 8192)))]
    pub description: Option<String>,

    /// the color of this tag
    #[cfg_attr(feature = "utoipa", schema(required = false))]
    pub color: Option<Color>,

    /// whether this tag is archived. cant be applied to any new threads or appear in pickers but still exists.
    pub is_archived: bool,
    // /// whether this tag is exclusive. functions similarly to forgejo
    // pub is_exclusive: bool,

    // /// restrict who can apply this tag. default: everyone
    // pub restrict: Option<Vec<RoleId>>,

    // /// if this tag includes other tags (composition). ie. tag `fruits` might include `apples` and `oranges`
    // // maybe don't include it here..?
    // pub includes: Option<Vec<TagId>>,
    // pub thread_count: u64,
    // pub active_thread_count: u64,
    // /// threads with this tag are hidden by default. does NOT prevent people from viewing those threads though, ie. not a permission.
    // pub is_default_hidden: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "validator", derive(Validate))]
pub struct TagCreate {
    pub room_id: RoomId,

    #[cfg_attr(feature = "utoipa", schema(min_length = 1, max_length = 64))]
    #[cfg_attr(feature = "validator", validate(length(min = 1, max = 64)))]
    pub name: String,

    #[cfg_attr(
        feature = "utoipa",
        schema(required = false, min_length = 1, max_length = 8192)
    )]
    #[cfg_attr(feature = "validator", validate(length(min = 1, max = 8192)))]
    pub description: Option<String>,

    /// the color of this tag
    #[cfg_attr(feature = "utoipa", schema(required = false))]
    pub color: Option<Color>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "validator", derive(Validate))]
pub struct TagPatch {
    #[cfg_attr(
        feature = "utoipa",
        schema(required = false, min_length = 1, max_length = 64)
    )]
    #[cfg_attr(feature = "validator", validate(length(min = 1, max = 64)))]
    pub name: Option<String>,

    #[cfg_attr(feature = "utoipa", schema(min_length = 1, max_length = 8192))]
    #[cfg_attr(feature = "validator", validate(length(min = 1, max = 8192)))]
    #[serde(default, deserialize_with = "some_option")]
    pub description: Option<Option<String>>,

    /// the color of this tag
    #[serde(default, deserialize_with = "some_option")]
    pub color: Option<Option<Color>>,

    /// whether this tag is archived. cant be applied to any new threads or appear in pickers but still exists.
    #[cfg_attr(feature = "utoipa", schema(required = false))]
    pub is_archived: Option<bool>,
}

/// something that can a tag can be applied to
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub enum Taggable {
    Room(Room),
    Thread(Thread),
    Tag(Tag),
}
