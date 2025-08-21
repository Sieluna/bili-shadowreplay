use serde::{Serialize, Deserialize};

/// HLS segment metadata (no actual data)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HlsSegment {
    /// Segment sequence number
    pub sequence: u64,
    /// Segment duration in seconds
    pub duration: f64,
    /// Segment URL for downloading
    pub url: String,
    /// Timestamp when segment was parsed
    pub timestamp: i64,
    /// Whether this segment has a discontinuity
    pub discontinuity: bool,
    /// Program date time if available
    pub program_date_time: Option<String>,
    /// Byte range if this is a partial segment (offset, length)
    pub byte_range: Option<(u64, u64)>,
}

impl HlsSegment {
    pub fn new(sequence: u64, duration: f64, url: String) -> Self {
        Self {
            sequence,
            duration,
            url,
            timestamp: chrono::Utc::now().timestamp_millis(),
            discontinuity: false,
            program_date_time: None,
            byte_range: None,
        }
    }

    pub fn with_discontinuity(mut self, discontinuity: bool) -> Self {
        self.discontinuity = discontinuity;
        self
    }

    pub fn with_program_date_time(mut self, program_date_time: Option<String>) -> Self {
        self.program_date_time = program_date_time;
        self
    }

    pub fn with_byte_range(mut self, byte_range: Option<(u64, u64)>) -> Self {
        self.byte_range = byte_range;
        self
    }

    /// Check if this segment is newer than another based on sequence
    pub fn is_newer_than(&self, other: &HlsSegment) -> bool {
        self.sequence > other.sequence
    }
}