use clap::Parser;
use rand_distr::{Alphanumeric, Distribution};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author="hnchengzong", 
    version="0.1.0", 
    about = "随机文件生成器", 
    long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 16)]
    number: usize,

    #[arg(short, long, default_value_t = 8)]
    length: usize,

    #[arg(short, long, default_value_t = 1024)]
    size: usize,

    #[arg(short = 'x', long, default_value = "txt")]
    suffix: Option<String>,

    #[arg(short = 'f', long, default_value_t = false)]
    force: bool,

    #[arg(short = 'r', long, default_value_t = false)]
    random_strings: bool,

    #[arg(num_args = 1..,default_value = ".")]
    pub dirs: Vec<PathBuf>,
}

pub fn random_string(len: usize) -> String {
    let mut rng = rand::rng();

    Alphanumeric
        .sample_iter(&mut rng)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn random_name(len: usize, suffix: Option<&str>) -> String {
    let name: String = random_string(len);
    if let Some(suffix) = suffix {
        format!("{}.{}", name, suffix)
    } else {
        name
    }
}

pub fn random_content(size: usize) -> Vec<u8> {
    random_string(size).into_bytes()
}

pub fn ensure_dir_exists(path: &str, force: bool) -> anyhow::Result<()> {
    let p = std::path::Path::new(path);
    if p.exists() {
        return Ok(());
    }
    if !force {
        println!("Directory {} does not exist, creating it?(y/N)", path);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            println!("Stop.");
            std::process::exit(0);
        }
    }

    fs::create_dir_all(p)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli: Cli = Cli::parse();

    if cli.random_strings {
        for _ in 0..cli.number {
            println!("Generate random string: {}", random_string(cli.length));
        }
        return Ok(());
    }

    println!(
        "Generate {} file(s) per directory with name length {}, size {} bytes, suffix: {:?}.",
        cli.number, cli.length, cli.size, cli.suffix
    );
    if !cli.force {
        println!("Do you want to continue? (y/n)");
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            println!("Stop.");
            return Ok(());
        }
    }

    for dir in cli.dirs.iter() {
        let dir_string = &dir.to_string_lossy().to_string();
        ensure_dir_exists(dir_string, cli.force)?;
        for _ in 0..cli.number {
            let file_name = random_name(cli.length, cli.suffix.as_deref());
            let file_path = dir.join(file_name);
            let file_content = random_content(cli.size);
            fs::write(file_path, file_content)?;
        }
    }

    Ok(())
}
