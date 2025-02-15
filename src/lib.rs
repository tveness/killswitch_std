#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

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
#[derive(Clone, Debug)]
pub struct KillSwitch {
    switch: Arc<AtomicBool>,
}

/// Derived from a [`KillSwitch`], allows to check if the kill switch is still alive, but cannot
/// activate it. This may be useful in separating out a thread which is only watching the value of
/// the kill switch.
#[derive(Clone, Debug)]
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
    pub fn kill(&self) -> Result<(), KillSwitchErr> {
        match self.is_alive() {
            true => {
                self.switch.store(false, Relaxed);
                Ok(())
            }
            false => Err(KillSwitchErr::AlreadyKilled),
        }
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

/// General error type for a [`KillSwitch`]
#[derive(Debug, Clone)]
pub enum KillSwitchErr {
    /// Kill switch has already been flipped
    AlreadyKilled,
}

impl std::error::Error for KillSwitchErr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for KillSwitchErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KillSwitchErr::AlreadyKilled => write!(f, "kill switch already killed"),
        }
    }
}
