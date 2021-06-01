use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo dropped");
    }
}

impl Future for Foo {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    let mut foo2 = Foo;

    tokio::select! {
        () = Foo => (),
        () = &mut foo2 => (),
        _ = tokio::time::sleep(std::time::Duration::from_millis(500)) => {
            println!("timed out");
        }
    }

    println!("after select");
}
