use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct L3CircuitBreaker {
    threshold: u32,
    cooldown: Duration,
    state: Mutex<CircuitState>,
}

struct CircuitState {
    consecutive_failures: u32,
    opened_at: Option<Instant>,
}

impl L3CircuitBreaker {
    pub fn new(threshold: u32, cooldown_secs: u64) -> Self {
        Self {
            threshold,
            cooldown: Duration::from_secs(cooldown_secs),
            state: Mutex::new(CircuitState {
                consecutive_failures: 0,
                opened_at: None,
            }),
        }
    }

    pub fn is_open(&self) -> bool {
        if self.threshold == 0 {
            return false; // disabled
        }
        let state = self.state.lock().unwrap();
        if state.consecutive_failures >= self.threshold
            && let Some(opened_at) = state.opened_at
        {
            return opened_at.elapsed() < self.cooldown;
        }
        false
    }

    pub fn record_success(&self) {
        let mut state = self.state.lock().unwrap();
        state.consecutive_failures = 0;
        state.opened_at = None;
    }

    pub fn record_failure(&self) {
        let mut state = self.state.lock().unwrap();
        state.consecutive_failures += 1;
        if state.consecutive_failures >= self.threshold && state.opened_at.is_none() {
            state.opened_at = Some(Instant::now());
            tracing::warn!(
                consecutive_failures = state.consecutive_failures,
                cooldown_secs = self.cooldown.as_secs(),
                "L3 circuit breaker OPENED — blocking upstream requests"
            );
        }
    }
}
