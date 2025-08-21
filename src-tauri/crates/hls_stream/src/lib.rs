pub mod stream;
pub mod segment;
pub mod provider;

use thiserror::Error;
use serde::{Serialize, Deserialize};

// Re-export main types
pub use stream::HlsStream;
pub use segment::HlsSegment;
pub use provider::{ProviderType, HlsProvider};

#[derive(Error, Debug)]
pub enum HlsStreamError {
    #[error("NetworkError: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("ParseError: {0}")]
    ParseError(String),
    #[error("AuthError: {0}")]
    AuthError(String),
    #[error("StreamOffline")]
    StreamOffline,
    #[error("UnsupportedPlatform: {0}")]
    UnsupportedPlatform(String),
    #[error("InitializationError: {0}")]
    InitializationError(String),
}

/// Stream events (pure metadata, no data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamEvent {
    /// New segment available
    NewSegment(HlsSegment),
    /// Stream started
    StreamStarted { 
        live_id: String, 
        playlist_url: String,
        target_duration: f64,
    },
    /// Stream ended
    StreamEnded,
    /// Quality changed
    QualityChanged { 
        from: String, 
        to: String, 
        new_playlist_url: String 
    },
    /// Playlist refreshed
    PlaylistRefreshed { 
        total_segments: usize, 
        new_segments: usize 
    },
}

/// Stream information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    pub live_id: String,
    pub current_quality: String,
    pub available_qualities: Vec<String>,
    pub playlist_url: String,
    pub target_duration: f64,
    pub is_live: bool,
    pub sequence_start: u64,
}