use clap::Parser;
use color_eyre::Result;
use crates::print_krates;
use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    args::Command,
    crates::{get_all_krates, upgrade_krates},
};

mod args;
mod crates;

fn main() -> Result<()> {
    let Command::ODeps(args) = Command::parse();
    let filter = if args.verbose {
        "cargo_odeps=debug,ureq=info"
    } else {
        "cargo_odeps=info,ureq=error"
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| filter.into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    debug!(?args);

    let ignore_local = !args.no_ignore_local;
    let krates = get_all_krates(&args.ignore, ignore_local, args.project)?;
    print_krates(&krates, args.outdated);
    if args.upgrade {
        upgrade_krates(&krates, &args.ignore, ignore_local)?;
    }

    Ok(())
}
