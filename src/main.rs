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
    #[arg(num_args = 1..,default_value = ".")]
    pub dirs: Vec<PathBuf>,

    #[arg(short, long, default_value_t = 16)]
    number: usize,

    #[arg(short, long, default_value_t = 8)]
    length: usize,

    #[arg(short, long, default_value_t = 1024)]
    size: usize,

    #[arg(short = 'x', long, default_value = "txt")]
    suffix: Option<String>,

    #[arg(long, default_value_t = false)]
    force: bool,
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
    let file_string: String = random_string(size);
    file_string.into_bytes().into_iter().take(size).collect()
}

pub fn ensure_dir_exists(path: &str) -> anyhow::Result<()> {
    if !std::path::Path::new(path).exists() {
        println!("Directory {} does not exist, creating it?(y/N)", path);
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            println!("Stop.");
            std::process::exit(0);
        }
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli: Cli = Cli::parse();

    println!(
        "It will generate {} file(s) with name length {}, size {} strings, suffix: {:?}.",
        cli.number, cli.length, cli.size, cli.suffix
    );
    println!("Do you want to continue? (y/n)");

    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input)?;
    if input.trim().to_lowercase() != "y" && !cli.force {
        println!("Stop.");
        return Ok(());
    }
    for dir in cli.dirs.iter() {
        let dir_string = dir.to_str().unwrap();
        ensure_dir_exists(dir_string)?;
        for _ in 0..cli.number {
            let file_name = random_name(cli.length, cli.suffix.as_deref());
            let file_path = dir.join(file_name);
            let file_content = random_content(cli.size);
            fs::write(file_path, file_content)?;
        }
    }

    Ok(())
}
