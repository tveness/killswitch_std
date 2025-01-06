#![deny(missing_docs)]

//! A simple implementation of a kill switch, using only the standard library
//!
//! # Example
//! ```
//!# tokio_test::block_on( async {
//! use std::time::Duration;
//! use killswitch_std::KillSwitch;
//!
//! // Create a kill switch
//! let kill = KillSwitch::default();
//!
//! println!("Is kill switch set? {}", kill);
//!
//! // Now make a couple of clones and check
//! let k1 = kill.watcher();
//! let t1 = tokio::spawn(async move {
//!     let duration = Duration::from_secs(2);
//!     for _ in 0..5 {
//!         tokio::time::sleep(duration).await;
//!         println!("Kill switch on thread 1: {}", k1);
//!     }
//!     println!("Thread 1 wrapping up");
//! });
//!
//! // Now make a couple of clones and check
//! let k2 = kill.watcher();
//! let t2 = tokio::spawn(async move {
//!     let duration = Duration::from_secs(2);
//!     for _ in 0..5 {
//!         tokio::time::sleep(duration).await;
//!         println!("Kill switch on thread 2: {}", k2);
//!     }
//!     println!("Thread 2 wrapping up");
//! });
//!
//! let duration = Duration::from_secs(7);
//! tokio::time::sleep(duration).await;
//! println!("Flipping kill switch on main thread");
//! let _ = kill.kill();
//!
//! let _ = tokio::join!(t1, t2);
//!
//! println!("All threads finished");
//!# })
//! ```

use std::{
    fmt::Display,
    sync::{
        atomic::{AtomicBool, Ordering::Relaxed},
        Arc,
    },
};

/// Convenience type which wraps a [`AtomicBool`].
/// Initially, `is_alive()` will return `true`. The value can be cloned across threads, and once it
/// has been `kill()`ed, then all of the clones will return `false` from `is_alive()`.
#[derive(Clone)]
pub struct KillSwitch {
    switch: Arc<AtomicBool>,
}

/// Derived from a [`KillSwitch`], allows to check if the kill switch is still alive, but cannot
/// activate it. This may be useful in separating out a thread which is only watching the value of
/// the kill switch.
#[derive(Clone)]
pub struct KillSwitchWatcher {
    switch: Arc<AtomicBool>,
}

impl KillSwitchWatcher {
    /// Check if the kill switch has been flipped. Before flipping will return `true`, and
    /// afterwards will return `false`
    pub fn is_alive(&self) -> bool {
        self.switch.load(Relaxed)
    }
}
impl KillSwitch {
    /// Check if the kill switch has been flipped. Before flipping will return `true`, and
    /// afterwards will return `false`
    pub fn is_alive(&self) -> bool {
        self.switch.load(Relaxed)
    }

    /// Flip the kill switch (will cause `is_alive()` to return `false`
    pub fn kill(&self) {
        self.switch.store(false, Relaxed)
    }

    /// Produce a kill switch which can only watch the value, but cannot flip the switch
    pub fn watcher(&self) -> KillSwitchWatcher {
        KillSwitchWatcher {
            switch: self.switch.clone(),
        }
    }
}

impl Default for KillSwitch {
    fn default() -> Self {
        Self {
            switch: Arc::new(AtomicBool::new(true)),
        }
    }
}

impl Display for KillSwitchWatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.is_alive() {
                true => "alive",
                false => "killed",
            }
        )
    }
}

impl Display for KillSwitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.is_alive() {
                true => "alive",
                false => "killed",
            }
        )
    }
}
