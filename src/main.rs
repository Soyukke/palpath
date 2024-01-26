use anyhow::Result;
use clap::{Parser, Subcommand};
use palpath::{calc::find_target_path, data::Data};

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
    /// パルのセットからターゲットのパルへ繋がるパスを見つける。見つからない場合は見つからないと表示する。
    Tree {
        /// male pal parents set
        #[arg(short = 'm', long, value_delimiter = ',')]
        males: Vec<String>,
        /// femal pal parents set
        #[arg(short = 'f', long, value_delimiter = ',')]
        females: Vec<String>,

        /// target pal
        #[arg(short = 't', long)]
        target: String,
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
            println!("start");
            for i in 0..n_iter {
                let (child, _v) = data.combine(&parent, &parent2)?;
                if parent2 == child {
                    println!("end");
                    break;
                }
                println!("step {}\t{}\tx\t{}\t=\t{}", i, parent, parent2, child);
                parent2 = child;
            }
        }
        SubCommands::Info { infocommands } => match infocommands {
            InfoCommands::Compact => {
                let data = Data::from_csv()?;
                data.find_compact()?;
            }
        },
        SubCommands::Tree {
            males,
            females,
            target,
        } => {
            find_target_path(
                males.iter().map(|s| s.as_str()).collect(),
                females.iter().map(|s| s.as_str()).collect(),
                &target,
            )?;
        }
    }

    Ok(())
}
