use clap::Parser;

/// Web server configuration
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(short = 'o', long)]
    pub output_directory: Option<String>,

    #[clap(short = 'n', long)]
    pub module_name: Option<String>,
}
