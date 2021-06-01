//! Showcases how tokio::select! cancels a branch by dropping the future associated with it.

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Foo dropped");
    }
}

async fn foo() -> Foo {
    let foo = Foo;
    std::future::pending::<()>().await;
    foo
}

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = foo() => {
            unreachable!();
        },
        _ = tokio::time::sleep(std::time::Duration::from_millis(500)) => {
            println!("timed out");
        }
    }

    println!("after select");
}
