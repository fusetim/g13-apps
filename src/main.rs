use app::error::Error as ErrorApp;
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
mod component;
mod display;
mod error;
mod style;

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

    // Wait for app to finish or user input (and do it forever)
    loop {
        select! {
            _app = app.execute(&mut output) => {
                app = match _app {
                    Ok(next) => next,
                    Err(error) => show_error(error),
                }
            }
            key = lines.next() => {
                if let Some(Ok(key)) = key {
                    let rst = match key.as_str() {
                        "BD" => app.button_bd().await,
                        "L1" => app.button_l1().await,
                        "L2" => app.button_l2().await,
                        "L3" => app.button_l3().await,
                        "L4" => app.button_l4().await,
                        _ => Ok(()),
                    };
                    if let Err(error) = rst {
                        app = show_error(error);
                    }
                }
            }
        }
    }
}

/// Open the G13 named pipes given.
/// Pipes are open in BufReader and BufWriter.
///
/// path_in is path for the g13 input pipe
/// and path_out is path for the g13 output pipe
async fn open_pipes<P: AsRef<Path>, Q: AsRef<Path>>(
    path_in: P,
    path_out: Q,
) -> Result<(BufReader<File>, BufWriter<File>), Error> {
    let pipe_in = OpenOptions::new().write(true).open(path_in).await?;
    let pipe_out = OpenOptions::new().read(true).open(path_out).await?;
    let output = BufWriter::new(pipe_in);
    let input = BufReader::new(pipe_out);
    Ok((input, output))
}

/// Print and create an Error app with the fiven error
fn show_error<E: std::error::Error>(error: E) -> App {
    eprintln!("Error: {}", error);
    let mut source : Option<&(dyn std::error::Error + 'static)> = error.source();
    while let Some(&err) = source.as_ref() {
        eprintln!("source: {}", err);
        source = err.source();
    }
    App::ErrorApp(ErrorApp::new(error))
}
