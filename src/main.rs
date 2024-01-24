use anyhow::Result;
use clap::Parser;
use palpath::data::Data;

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct Cli {
    /// parent 1, fix parent.
    #[arg(short = 'p', long)]
    parent1: String,
    /// parent 2
    #[arg(short = 'q', long)]
    parent2: String,
    /// parent 2
    #[arg(short, long, default_value_t = 10)]
    n_iter: usize,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let data = Data::from_csv()?;
    let parent = args.parent1;
    let mut parent2 = args.parent2;
    for i in 0..args.n_iter {
        let (child, _v) = data.combine(&parent, &parent2);
        println!("Step {}\t{}\tx\t{}\t=\t{}", i, parent, parent2, child);
        parent2 = child;
    }

    Ok(())
}
