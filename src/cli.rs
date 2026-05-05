use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author = "hnchengzong", version = "0.1.0", about = "随机文件生成器")]
pub struct Cli {
    #[arg(short, long, default_value_t = 16)]
    pub file_count: usize,

    #[arg(short, long, default_value_t = 8)]
    pub name_len: usize,

    #[arg(short, long, default_value_t = 1024)]
    pub file_size: usize,

    #[arg(short = 'x', long)]
    pub ext: Option<String>,

    #[arg(short = 'f', long, default_value_t = false)]
    pub force_create: bool,

    #[arg(short = 'r', long, default_value_t = false)]
    pub just_strings: bool,

    #[arg(num_args = 1.., default_value = ".")]
    pub dirs: Vec<PathBuf>,
}

impl Cli {
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.file_count <= 0 {
            anyhow::bail!("The number of files must be greater than 0");
        }
        if self.name_len <= 0 || self.name_len > 1024 {
            anyhow::bail!("The length of the filename must be between 1 and 1024");
        }
        if self.file_size <= 0 {
            anyhow::bail!("The file size must be greater than 0");
        }
        Ok(())
    }
}