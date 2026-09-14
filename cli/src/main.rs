use cli::initate_programm;
use cli::args::Args;
use clap::Parser;
use library_core::core_error::CoreError;

fn main() {
    let args = Args::parse();
    match initate_programm(args) {
        Ok(_) => (),
        Err(err) => 
    }
}
