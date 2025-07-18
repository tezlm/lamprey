use std::{sync::Arc, time::Instant};

use common::v1::types::{
    voice::{SignallingMessage, VoiceState},
    ThreadId, UserId,
};
use serde::{Deserialize, Serialize};
use str0m::{
    format::PayloadParams,
    media::{MediaKind, MediaTime, Mid},
};

pub mod config;
pub mod peer;
pub mod sfu;
pub mod util;

#[derive(Debug, Serialize, Deserialize)]
pub struct SfuCommand {
    /// the user who sent this, or None if this is from the server
    pub user_id: Option<UserId>,

    #[serde(flatten)]
    pub inner: SignallingMessage,
}

#[derive(Debug, serde::Serialize)]
#[serde(tag = "type")]
pub enum SfuEvent {
    VoiceDispatch {
        user_id: UserId,
        payload: SignallingMessage,
    },
    VoiceState {
        user_id: UserId,
        old: Option<VoiceState>,
        state: Option<VoiceState>,
    },
}

#[derive(Debug)]
pub struct PeerEventEnvelope {
    pub user_id: UserId,
    pub payload: PeerEvent,
}

#[derive(Debug)]
pub enum PeerEvent {
    Signalling(SignallingMessage),
    MediaAdded(SfuTrack),
    MediaData(MediaData),
    Dead,
}

#[derive(Debug)]
pub enum PeerCommand {
    Signalling(SignallingMessage),
    MediaAdded(SfuTrack),
    MediaData(MediaData),
    Kill,
}

#[derive(Debug, Clone)]
pub struct MediaData {
    pub mid: Mid,
    pub peer_id: UserId,
    pub network_time: Instant,
    pub time: MediaTime,
    pub data: Arc<[u8]>,
    pub params: PayloadParams,
}

#[derive(Debug, Clone)]
pub struct SfuTrack {
    pub mid: Mid,
    pub peer_id: UserId,
    pub thread_id: ThreadId,
    pub kind: MediaKind,
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrackState {
    Pending,
    Negotiating(Mid),
    Open(Mid),
}

impl TrackState {
    pub fn mid(&self) -> Option<Mid> {
        match self {
            TrackState::Pending => None,
            TrackState::Negotiating(mid) => Some(*mid),
            TrackState::Open(mid) => Some(*mid),
        }
    }
}

#[derive(Debug)]
pub struct TrackIn {
    pub kind: MediaKind,
    pub state: TrackState,
    pub thread_id: ThreadId,
    pub key: String,
}

#[derive(Debug)]
pub struct TrackOut {
    pub kind: MediaKind,
    pub state: TrackState,
    pub peer_id: UserId,
    pub source_mid: Mid,
    pub enabled: bool,
    pub needs_keyframe: bool,
    pub thread_id: ThreadId,
    pub key: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// no voice state exists for this user
    #[error("no voice state exists for this user")]
    NotConnected,
}

// impl From<MediaKindSerde> for MediaKind {
//     fn from(kind: MediaKindSerde) -> Self {
//         match kind {
//             MediaKindSerde::Audio => MediaKind::Audio,
//             MediaKindSerde::Video => MediaKind::Video,
//         }
//     }
// }

// impl From<MediaKind> for MediaKindSerde {
//     fn from(kind: MediaKind) -> Self {
//         match kind {
//             MediaKind::Audio => MediaKindSerde::Audio,
//             MediaKind::Video => MediaKindSerde::Video,
//         }
//     }
// }
