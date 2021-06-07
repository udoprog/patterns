use tokio::time;

#[tokio::main]
async fn main() {
    let test = time::sleep(time::Duration::from_millis(100));
    tokio::pin!(test);

    // Self-referential types that make use of intrusive collections are
    // currently unsound. If you run this with MIRI you'll as of 2021-06 get an
    // error.
    test.as_mut().await;
}
