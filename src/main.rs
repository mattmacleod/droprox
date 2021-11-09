use clap::{Parser};

/// Droprox is an open-source Dropbox sync client written in Rust
#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Matthew MacLeod <matt@matt-m.co.uk>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    Version(Version),
}

/// A subcommand for displaying the app version
#[derive(Parser)]
struct Version {
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Version(_) => {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        }
    }
}
