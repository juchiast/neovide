use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use log::info;

pub static RUNNING_TRACKER: RunningTracker = RunningTracker::new();

pub struct RunningTracker {
    running: AtomicBool,
    exit_code: AtomicI32,
}

impl RunningTracker {
    const fn new() -> Self {
        Self {
            running: AtomicBool::new(true),
            exit_code: AtomicI32::new(0),
        }
    }

    pub fn quit(&self, reason: &str) {
        self.running.store(false, Ordering::Relaxed);
        info!("Quit {}", reason);
    }

    pub fn quit_with_code(&self, code: i32, reason: &str) {
        self.exit_code.store(code, Ordering::Relaxed);
        self.running.store(false, Ordering::Relaxed);
        info!("Quit with code {}: {}", code, reason);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn exit_code(&self) -> i32 {
        self.exit_code.load(Ordering::Relaxed)
    }
}
