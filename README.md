# killswitch_std

[![Crates.io](https://img.shields.io/crates/v/killswitch_std.svg?style=for-the-badge)](https://crates.io/crates/killswitch_std)
[![Documentation](https://img.shields.io/docsrs/killswitch_std?style=for-the-badge)](https://docs.rs/killswitch_std/)
[![Build status](https://img.shields.io/github/actions/workflow/status/tveness/killswitch_std/rust.yml?label=Tests&style=for-the-badge
)](https://github.com/tveness/killswitch_std/actions/workflows/rust.yml)
[![License](https://img.shields.io/github/license/tveness/killswitch_std?style=for-the-badge)](https://www.gnu.org/licenses/agpl-3.0.en.html)


killswitch_std is a simple crate with no dependencies outside of the
standard library for creating a thread-safe kill switch


# Example

```rust, no_compile
#[tokio::main]
async fn main() {
    use killswitch_std::KillSwitch;
    use std::time::Duration;

    // Create a kill switch
    let kill = KillSwitch::default();

    println!("Is kill switch set? {}", kill);

    // Now make a couple of clones and check
    let k1 = kill.watcher();
    let t1 = tokio::spawn(async move {
        let duration = Duration::from_secs(2);
        for _ in 0..5 {
            tokio::time::sleep(duration).await;
            println!("Kill switch on thread 1: {}", k1);
        }
        println!("Thread 1 wrapping up");
    });

    // Now make a couple of clones and check
    let k2 = kill.watcher();
    let t2 = tokio::spawn(async move {
        let duration = Duration::from_secs(2);
        for _ in 0..5 {
            tokio::time::sleep(duration).await;
            println!("Kill switch on thread 2: {}", k2);
        }
        println!("Thread 2 wrapping up");
    });

    let duration = Duration::from_secs(7);
    tokio::time::sleep(duration).await;
    println!("Flipping kill switch on main thread");
    let _ = kill.kill();

    let _ = tokio::join!(t1, t2);

    println!("All threads finished");
}
 ```
 
Should produce output of the following form:
```text
Is kill switch set? alive
Kill switch on thread 2: alive
Kill switch on thread 1: alive
Kill switch on thread 2: alive
Kill switch on thread 1: alive
Kill switch on thread 2: alive
Kill switch on thread 1: alive
Flipping kill switch on main thread
Kill switch on thread 2: killed
Kill switch on thread 1: killed
Kill switch on thread 1: killed
Thread 1 wrapping up
Kill switch on thread 2: killed
Thread 2 wrapping up
All threads finished
```
 

