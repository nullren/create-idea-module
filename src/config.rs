use clap::{Parser, ValueEnum};

/// Web server configuration
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    #[clap(short = 'o', long)]
    pub output_directory: Option<String>,

    #[clap(short = 'n', long)]
    pub module_name: Option<String>,

    #[clap(short = 'l', arg_enum, default_value_t = ModuleLanguage::Rust)]
    pub module_language: ModuleLanguage,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ModuleLanguage {
    Go,
    Rust,
    Web,
}
