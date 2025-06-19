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

fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    match (args.all, args.metadata, args.verbose) {
        (true, true, true) => nls::list::list(args.all, args.metadata, args.verbose),
        (true, true, false) => nls::list::list(args.all, args.metadata, args.verbose),
        (true, false, false) => nls::list::list(args.all, args.metadata, args.verbose),
        (false, false, false) => nls::list::list(args.all, args.metadata, args.verbose),
        _ => todo!(),
    }

}
