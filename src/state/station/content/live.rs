use chrono::{DateTime, Duration, Utc};
pub struct LiveStream {
    url:String,
    start:DateTime<Utc>,
    delay:Option<Duration>,
    duration:Option<Duration>,
    host:String //probably replace this with enum
}

impl PartialEq for LiveStream {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
    }
}

impl Eq for LiveStream {}

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
