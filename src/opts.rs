use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "rust_cli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "show csv or convert csv to other format")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short = 'i', long, help = "input csv file", value_parser = verify_input_file)]
    input: String,

    #[arg(
        short = 'o',
        long,
        help = "output csv file",
        default_value = "assets/output.json"
    )]
    output: String,

    #[arg(short = 'd', long, help = "delimiter", default_value_t = ',')]
    delimiter: char,

    #[arg(long, help = "header", default_value_t = true)]
    header: bool,
}

impl CsvOpts {
    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn get_output(&self) -> &str {
        &self.output
    }
}

fn verify_input_file(input_file_name: &str) -> Result<String, String> {
    if Path::new(input_file_name).exists() {
        Ok(input_file_name.into())
    } else {
        Err(format!("Input file {} does not exist", input_file_name))
    }
}
