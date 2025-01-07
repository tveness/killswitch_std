use killswitch_std::KillSwitch;
use std::time::Duration;
use tokio::task::JoinSet;

#[tokio::test]
async fn multiple_watchers() {
    // Create a kill switch
    let kill = KillSwitch::default();

    assert!(kill.is_alive());

    let mut js = JoinSet::new();
    // Generate 10 threads
    for t in 0..10 {
        // Now make some watchers and monitor the kill switch
        let w = kill.watcher();
        js.spawn(async move {
            let duration = Duration::from_secs(2);

            // Should be alive for first two loops
            for _ in 0..2 {
                tokio::time::sleep(duration).await;
                println!("Kill switch on thread {t}: {}", w);
                assert!(w.is_alive());
            }

            // Should be flipped for second two loops
            for _ in 0..2 {
                tokio::time::sleep(duration).await;
                println!("Kill switch on thread {t}: {}", w);
                assert!(!w.is_alive());
            }

            println!("Thread {t} wrapping up");
        });
    }

    // Wait 5 seconds and then flip the switch
    js.spawn(async move {
        let duration = Duration::from_secs(5);
        tokio::time::sleep(duration).await;
        println!("Flipping kill switch on auxiliary thread");
        let _ = kill.kill();
    });

    js.join_all().await;

    println!("All threads finished");
}

#[test]
#[should_panic]
fn double_flip() {
    let k = KillSwitch::default();

    let _ = k.kill();

    k.kill().unwrap();
}
