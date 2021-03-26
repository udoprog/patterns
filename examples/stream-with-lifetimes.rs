use std::pin::Pin;
use tokio_stream::Stream;
use tokio_stream::StreamExt as _;

#[derive(Debug)]
struct Foo;

fn foo<'a>(value: &'a Foo) -> Pin<Box<dyn Stream<Item = usize> + 'a>> {
    Box::pin(async_stream::stream! {
        for i in 0..5usize {
            println!("{:?}", value); // <- we're borrowing value for the duration of this stream!
            yield i;
        }
    })
}

#[tokio::main]
async fn main() {
    let value = Foo;

    {
        let mut stream = foo(&value);

        while let Some(n) = stream.next().await {
            println!("number: {}", n);
        }
    }

    drop(value); // <- value is no longer in use, so we can drop it.
}
