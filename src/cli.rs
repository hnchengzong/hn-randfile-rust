use clap::Parser;
use std::path::PathBuf;



#[derive(Parser)]
#[command(author = "hnchengzong",
version = env!("CARGO_PKG_VERSION"),
about = "Random File Generator",
long_about = "Generates a specified number of files filled with random data.")]
pub struct Cli {
    #[arg(short = 'c', long, default_value_t = 16,value_parser = clap::value_parser!(u64).range(1..),help = "Number of files")]
    pub file_count: usize,

    #[arg(short = 'n', long,default_value_t = 8, value_parser = clap::value_parser!(u64).range(1..=1024),help = "Length of the filename")]
    pub name_len: usize,

    #[arg(short = 's', long, default_value_t = 1024,value_parser = clap::value_parser!(u64).range(1..),help = "Size of each file in bytes" )]
    pub file_size: usize,

    #[arg(short = 'x', long,help = "File extension")]
    pub ext: Option<String>,

    #[arg(short = 'f', long, default_value_t = false,help = "Force overwrite")]
    pub force_create: bool,

    #[arg(short = 'r', long, default_value_t = false,help = "Only print random strings to stdout")]
    pub just_strings: bool,

    #[arg(num_args = 1.., default_value = ".",help = "Target directories")]
    pub dirs: Vec<PathBuf>,
}

