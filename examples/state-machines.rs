use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

enum AsyncFnState {
    Initial,
    Second,
    Complete,
}

fn async_fn_1(state: &mut AsyncFnState) -> Poll<u32> {
    match state {
        AsyncFnState::Initial => {
            println!("first");
            *state = AsyncFnState::Second;
            return Poll::Pending;
        }
        AsyncFnState::Second => {
            println!("second");
            *state = AsyncFnState::Complete;
            return Poll::Ready(42);
        }
        AsyncFnState::Complete => {
            panic!("polled after completion");
        }
    }
}

fn manual_state_machine() {
    let mut state = AsyncFnState::Initial;

    loop {
        match async_fn_1(&mut state) {
            Poll::Pending => {
                /* put task to sleep until it's ready to make progress */
                println!("pending");
            }
            Poll::Ready(n) => {
                println!("completed with: {}", n);
                break;
            }
        }
    }
}

async fn async_fn_2() -> u32 {
    println!("first");
    // E.g. a future that blocks once.
    BlockOnce::default().await;
    println!("second");
    42
}

fn automatic_state_machine() {
    let task2 = async_fn_2();
    tokio::pin!(task2);

    let waker = fake_waker();
    let mut cx = Context::from_waker(&waker);

    loop {
        match task2.as_mut().poll(&mut cx) {
            Poll::Pending => {
                /* put task to sleep until it's ready to make progress */
                println!("pending");
            }
            Poll::Ready(n) => {
                println!("completed with: {}", n);
                break;
            }
        }
    }
}

fn main() {
    manual_state_machine();
    automatic_state_machine();
}

// helpers below here!

#[derive(Default)]
struct BlockOnce {
    blocked: bool,
}

impl Future for BlockOnce {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.blocked {
            self.blocked = true;
            return Poll::Pending;
        }

        Poll::Ready(())
    }
}

/// Construct a fake raw waker to use in examples.
fn fake_raw_waker() -> RawWaker {
    static VTABLE: &RawWakerVTable = &RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    return RawWaker::new(std::ptr::null(), &VTABLE);

    unsafe fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    unsafe fn wake(_: *const ()) {}

    unsafe fn wake_by_ref(_: *const ()) {}

    unsafe fn drop(_: *const ()) {}
}

fn fake_waker() -> Waker {
    // Safety: Context doesn't do anything.
    unsafe { Waker::from_raw(fake_raw_waker()) }
}
