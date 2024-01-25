use anyhow::Result;
use clap::{Parser, Subcommand};
use palpath::{calc::find_compact, data::Data};

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    #[clap(arg_required_else_help = true)]
    Info {
        #[command(subcommand)]
        infocommands: InfoCommands,
    },
    Dig {
        /// parent 1, fix parent.
        #[arg(short = 'p', long)]
        parent1: String,
        /// parent 2
        #[arg(short = 'q', long)]
        parent2: String,
        /// parent 2
        #[arg(short, long, default_value_t = 10)]
        n_iter: usize,
    },
}

#[derive(Debug, Subcommand)]
enum InfoCommands {
    Compact,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.subcommand {
        SubCommands::Dig {
            parent1,
            parent2,
            n_iter,
        } => {
            let data = Data::from_csv()?;
            let parent = parent1;
            let mut parent2 = parent2;
            for i in 0..n_iter {
                let (child, _v) = data.combine(&parent, &parent2);
                println!("Step {}\t{}\tx\t{}\t=\t{}", i, parent, parent2, child);
                parent2 = child;
            }
        }
        SubCommands::Info { infocommands } => match infocommands {
            InfoCommands::Compact => {
                let data = Data::from_csv()?;
                data.find_compact();
            }
        },
    }

    Ok(())
}
