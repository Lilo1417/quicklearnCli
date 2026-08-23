use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub database: Option<PathBuf>,
}
