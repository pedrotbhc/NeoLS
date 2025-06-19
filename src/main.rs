use clap::Parser;
use std::io;

mod nls;
 
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    all: bool,
    #[arg(short, long)]
    metadata: bool,
    #[arg(long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    nls::list::list(args.all, args.metadata, args.verbose);

}
