use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    use tokio::io::AsyncBufReadExt as _;
    use tokio::{fs, io};

    let file = fs::File::open("sample.txt").await?;
    let mut file = io::BufReader::new(file);

    let mut line = String::new();

    loop {
        line.clear();

        if file.read_line(&mut line).await? == 0 {
            break;
        }

        dbg!(&line);
    }

    Ok(())
}
