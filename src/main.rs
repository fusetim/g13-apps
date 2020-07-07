use app::App;
use app::Application;
use error::Error;
use std::path::Path;
use std::str::FromStr;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::io::BufWriter;
use tokio::select;
use tokio::stream::StreamExt;

mod app;
mod display;
mod error;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // Open the G13 driver named pipes
    let (input, mut output) = open_pipes(env!("G13_IN"), env!("G13_OUT"))
        .await
        .expect("Can't open the communication pipe with the g13 driver!");
    let mut lines = input.lines();

    // Start the menu app
    let mut app = App::from_str("menu").expect("Menu app should exist!");

    // Wait for app to finish or user input (and restart again)
    loop {
        select! {
            _app = app.execute(&mut output) => {
                app = _app.unwrap()
            }
            key = lines.next() => {
                if let Some(Ok(key)) = key {
                    match key.as_str() {
                        "BD" => app.button_bd().await.unwrap(),
                        "L1" => app.button_l1().await.unwrap(),
                        "L2" => app.button_l2().await.unwrap(),
                        "L3" => app.button_l3().await.unwrap(),
                        "L4" => app.button_l4().await.unwrap(),
                        _ => {},
                    }
                }
            }
        }
    }
}


/// Open the G13 named pipes given.
/// Pipes are open in BufReader annd BufWriter.
/// 
/// path_in is path for the g13 input pipe
/// and path_out is path for the g13 output pipe
async fn open_pipes<P: AsRef<Path>, Q: AsRef<Path>>(
    path_in: P,
    path_out: Q,
) -> Result<(BufReader<File>, BufWriter<File>), Error> {
    let pipe_in = OpenOptions::new()
        .write(true)
        .open(path_in)
        .await?;
    let pipe_out = OpenOptions::new()
        .read(true)
        .open(path_out)
        .await?;
    let output = BufWriter::new(pipe_in);
    let input = BufReader::new(pipe_out);
    Ok((input, output))
}
