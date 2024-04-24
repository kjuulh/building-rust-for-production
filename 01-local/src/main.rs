use std::{collections::HashMap, net::SocketAddr};

use axum::{routing::get, Router};
use clap::{Parser, Subcommand, ValueEnum};
use serde::Deserialize;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cmd = Command::parse();

    match cmd.commands {
        Commands::HelloWorld { format } => match format {
            OutputFormat::Text => println!("Hello, world!"),
            OutputFormat::Json => println!(
                "{}",
                serde_json::json!({
                    "message": "Hello, world!",
                })
            ),
        },
        Commands::Fetch {} => {
            #[derive(Deserialize, Clone, Debug)]
            struct HttpResp {
                headers: HashMap<String, String>,
            }

            let resp: HttpResp = reqwest::get("https://httpbin.org/get")
                .await?
                .json()
                .await?;

            println!("httpbin responded with headers: {}", resp.headers.len());
        }
        Commands::Serve { host } => {
            let app = Router::new().route("/", get(|| async move { "Hello, world!" }));

            tracing::info!("listening on {}", &host);
            let listener = tokio::net::TcpListener::bind(&host).await?;
            axum::serve(listener, app.into_make_service()).await?;
        }
        Commands::LocalRepo {} => {
            let repo = git2::Repository::open(".")?;

            for remote in repo.remotes()?.iter().flatten() {
                println!("found remote: {}", remote);
            }
        }
        Commands::Bench {} => {
            #[inline(never)]
            fn calc(x: usize) -> usize {
                x + 1
            }

            let func = |x: usize, y: usize| x + calc(y);

            let mut count = 0;

            let timer = std::time::Instant::now();
            for i in 0..1_000_000_000 {
                count = func(i, count)
            }

            let took = timer.elapsed();
            println!(
                "it took: {} milliseconds to calculate: {}",
                took.as_millis(),
                count
            );
        }
    }

    Ok(())
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None, subcommand_required = true)]
struct Command {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Clone, Debug, Subcommand)]
enum Commands {
    HelloWorld {
        #[arg(long = "format", default_value = "text")]
        format: OutputFormat,
    },
    Fetch {},
    LocalRepo {},
    Serve {
        #[arg(long = "host", default_value = "127.0.0.1:3000")]
        host: SocketAddr,
    },
    Bench {},
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Text,
    Json,
}
