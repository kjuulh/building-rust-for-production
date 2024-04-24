use clap::{Parser, Subcommand};

fn main() {
    let cmd = Command::parse();

    match cmd.commands {
        Commands::HelloWorld {} => {
            println!("Hello, world!");
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
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None, subcommand_required = true)]
struct Command {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Clone, Debug, Subcommand)]
enum Commands {
    HelloWorld {},
    Bench {},
}
