use std::{
    fmt::Display,
    marker::PhantomData,
    sync::{
        atomic::{AtomicBool, Ordering::Relaxed},
        Arc,
    },
};

/// Convenience type which wraps a [`OnceCell`].
/// Initially, `is_alive()` will return `true`. The value can be cloned across threads, and once it
/// has been `kill()`ed, then all of the clones will return `false` from `is_alive()`.
#[derive(Clone)]
struct KillSwitch<T = Switcher>
where
    T: KillSwitchType,
{
    switch: Arc<AtomicBool>,
    _marker: PhantomData<T>,
}

impl<T: KillSwitchType> KillSwitch<T> {
    /// Check if the kill switch has been flipped. Before flipping will return `true`, and
    /// afterwards will return `false`
    pub fn is_alive(&self) -> bool {
        self.switch.load(Relaxed)
    }

    // Create a new, un-flipped kill switch
    pub fn new() -> Self {
        Self {
            switch: Arc::new(AtomicBool::new(true)),
            _marker: PhantomData,
        }
    }
}

#[derive(Clone)]
struct Switcher;
#[derive(Clone)]
struct Watcher;
trait KillSwitchType {}

impl KillSwitchType for Switcher {}
impl KillSwitchType for Watcher {}

impl KillSwitch<Switcher> {
    /// Flip the kill switch (will cause `is_alive()` to return `false`
    pub fn kill(&self) {
        self.switch.store(false, Relaxed)
    }

    /// Produce a kill switch which can only watch the value, but cannot flip the switch
    pub fn watcher(&self) -> KillSwitch<Watcher> {
        KillSwitch {
            switch: self.switch.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T: KillSwitchType> Display for KillSwitch<T> {
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
