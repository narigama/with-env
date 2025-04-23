use clap::Parser;

/// A simple utility to load .env before starting a command, for the lifetime of the command.
#[derive(Debug, Parser)]
pub struct Args {
    /// an env file to load, looks for .env otherwise
    #[clap(long)]
    env_file: Option<std::path::PathBuf>,

    #[clap()]
    command: Vec<String>,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    // load env
    match &args.env_file {
        Some(path) => dotenvy::from_path(path)?,
        None => {
            dotenvy::dotenv().ok();
        }
    }

    // run program
    let status = execute::shell(args.command.clone().join(" "))
        .spawn()?
        .wait()?;

    // mirror it's exit code
    std::process::exit(status.code().unwrap_or(0));
}
