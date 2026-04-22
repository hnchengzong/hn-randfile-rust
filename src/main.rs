use clap::Parser;
use rand::Rng;
use rand_distr::{Alphanumeric, Distribution};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author="hnchengzong", version="0.1.0", about = "随机文件生成器", long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 16)]
    file_count: usize,

    #[arg(short, long, default_value_t = 8)]
    name_len: usize,

    #[arg(short, long, default_value_t = 1024)]
    file_size: usize,

    #[arg(short = 'x', long, default_value = "txt")]
    ext: Option<String>,

    #[arg(short = 'f', long, default_value_t = false)]
    force_create: bool,

    #[arg(short = 'r', long, default_value_t = false)]
    just_strings: bool,

    #[arg(num_args = 1.., default_value = ".")]
    dirs: Vec<PathBuf>,
}

fn confirm(prompt: &str, force: bool) -> anyhow::Result<bool> {
    if force {
        return Ok(true);
    }
    println!("{} (y/n)", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase() == "y")
}

fn random_name(rng: &mut impl Rng, len: usize, ext: Option<&str>) -> String {
    let name: String = Alphanumeric
        .sample_iter(rng)
        .take(len)
        .map(char::from)
        .collect();
    if let Some(ext) = ext {
        format!("{}.{}", name, ext)
    } else {
        name
    }
}

fn random_content(rng: &mut impl Rng, size: usize) -> Vec<u8> {
    Alphanumeric
        .sample_iter(rng)
        .take(size)
        .map(char::from)
        .collect::<String>()
        .into_bytes()
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut rng = rand::rng();

    if cli.just_strings {
        for _ in 0..cli.file_count {
            let s: String = Alphanumeric
                .sample_iter(&mut rng)
                .take(cli.name_len)
                .map(char::from)
                .collect();
            println!("{}", s);
        }
        return Ok(());
    }

    println!(
        "Generate {} file(s) per directory with name length {}, size {} bytes, suffix: {:?}.",
        cli.file_count, cli.name_len, cli.file_size, cli.ext
    );

    if !confirm("Do you want to continue?", cli.force_create)? {
        println!("Stop.");
        return Ok(());
    }

    for dir in &cli.dirs {
        fs::create_dir_all(dir)?;
        for _ in 0..cli.file_count {
            let filename = random_name(&mut rng, cli.name_len, cli.ext.as_deref());
            let path = dir.join(filename);
            let content = random_content(&mut rng, cli.file_size);
            fs::write(path, content)?;
        }
    }

    Ok(())
}
