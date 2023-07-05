//! Handle `cargo odeps` arguments
use clap::Parser;

#[derive(Debug, Parser)]
#[command(bin_name = "cargo")]
pub enum Command {
    /// Easy manage rust crate deps by `cargo odeps`.
    #[command(name = "odeps")]
    ODeps(Args),
}

#[derive(Debug, Default, Parser)]
pub struct Args {
    /// upgrade all outdated
    #[arg(short, long)]
    pub upgrade: bool,

    /// show outdated
    #[arg(short, long)]
    pub outdated: bool,

    /// don't ignore deps from local space
    #[arg(short = 'l', long)]
    pub no_ignore_local: bool,

    /// ignore from upgrade
    #[arg(short, long)]
    pub ignore: Vec<String>,

    /// project only
    #[arg(short, long)]
    pub project: Option<String>,

    /// show details
    #[arg(short, long)]
    pub verbose: bool,
}
