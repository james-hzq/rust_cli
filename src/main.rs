use anyhow::Error;
use clap::Parser;
use rust_cli::{Opts, SubCommand, process_csv_to_json};

fn main() -> Result<(), Error> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv_to_json(opts.get_input(), opts.get_output())?;
        }
    }

    Ok(())
}
