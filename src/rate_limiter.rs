use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct L3RateLimiter {
    max_rpm: u64,
    timestamps: Mutex<VecDeque<Instant>>,
}

impl L3RateLimiter {
    pub fn new(max_rpm: u64) -> Self {
        Self {
            max_rpm,
            timestamps: Mutex::new(VecDeque::new()),
        }
    }

    pub fn try_acquire(&self) -> bool {
        if self.max_rpm == 0 {
            return true; // disabled
        }
        let now = Instant::now();
        let window = Duration::from_secs(60);
        let mut timestamps = self.timestamps.lock().unwrap();
        // Prune old entries
        while let Some(front) = timestamps.front() {
            if now.duration_since(*front) > window {
                timestamps.pop_front();
            } else {
                break;
            }
        }
        if (timestamps.len() as u64) < self.max_rpm {
            timestamps.push_back(now);
            true
        } else {
            false
        }
    }
}
