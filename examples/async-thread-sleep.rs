//! A very simple async sleep implementation that spawns and blocks a thread to
//! implement its sleep.

use pin_project::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::thread;
use std::time;

#[pin_project]
struct ThreadSleep {
    duration: time::Duration,
    needs_init: bool,
    was_woken: Arc<AtomicBool>,
}

impl ThreadSleep {
    fn new(duration: time::Duration) -> Self {
        Self {
            duration,
            needs_init: true,
            was_woken: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Future for ThreadSleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let waker = cx.waker().clone();
        let was_woken = this.was_woken.clone();

        if std::mem::take(this.needs_init) {
            let duration = *this.duration;

            // a really sad reactor
            thread::spawn(move || {
                thread::sleep(duration);
                was_woken.store(true, Ordering::SeqCst);
                waker.wake();
            });
        }

        if this.was_woken.load(Ordering::SeqCst) {
            return Poll::Ready(());
        }

        Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    let started = time::Instant::now();
    ThreadSleep::new(time::Duration::from_secs(1)).await;

    println!(
        "ended after: {:?}",
        time::Instant::now().duration_since(started)
    );
}
