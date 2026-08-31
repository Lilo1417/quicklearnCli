use cli::initate_programm;
use cli::args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    match initate_programm(args) {
        Ok(_) => (),
        Err(err) => panic!("{}", err)
    }
}
