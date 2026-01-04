use chrono::{DateTime, Duration, Utc};

/// Scheduled live stream with timing information
pub struct LiveStream {
    location: String,             // Stream URL
    start: DateTime<Utc>,         // Scheduled start time
    delay: Option<Duration>,      // Optional delay before stream starts
    duration: Option<Duration>,   // Max duration before cutting to static (avoids ads/premium)
    host: String                  // Stream host/provider (TODO: replace with enum)
}

impl PartialEq for LiveStream {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
    }
}

impl Eq for LiveStream {}

// LiveStreams are ordered by start time for scheduling
impl Ord for LiveStream {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for LiveStream {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
