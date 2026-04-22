use clap::Parser;
use rand::Rng;
use rand_distr::{Alphanumeric, Distribution};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author = "hnchengzong", version = "0.1.0", about = "随机文件生成器")]
struct Cli {
    #[arg(short, long, default_value_t = 16)]
    file_count: usize,

    #[arg(short, long, default_value_t = 8)]
    name_len: usize,

    #[arg(short, long, default_value_t = 1024)]
    file_size: usize,

    #[arg(short = 'x', long)]
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
    Ok(input.trim().eq_ignore_ascii_case("y"))
}

fn random_name(rng: &mut impl Rng, len: usize, ext: Option<&str>) -> String {
    let name: String = Alphanumeric
        .sample_iter(rng)
        .take(len)
        .map(char::from)
        .collect();

    match ext {
        Some(e) => format!("{}.{}", name, e),
        None => name,
    }
}

fn random_content(rng: &mut impl Rng, size: usize) -> Vec<u8> {
    Alphanumeric
        .sample_iter(rng)
        .take(size)
        .map(|c| c as u8)
        .collect()
}

fn handle_print_strings(file_count: usize, name_len: usize) {
    let mut rng = rand::rng();
    for _ in 0..file_count {
        let s: String = Alphanumeric
            .sample_iter(&mut rng)
            .take(name_len)
            .map(char::from)
            .collect();
        println!("{}", s);
    }
}

fn handle_generate_files(
    dirs: &[PathBuf],
    file_count: usize,
    name_len: usize,
    file_size: usize,
    ext: Option<&str>,
    force_create: bool,
) -> anyhow::Result<()> {
    println!(
        "Generate {} file(s) per directory with name length {}, size {} bytes, suffix: {:?}.",
        file_count, name_len, file_size, ext
    );

    if !confirm("Do you want to continue?", force_create)? {
        println!("Stop.");
        return Ok(());
    }

    let mut rng = rand::rng();
    for dir in dirs {
        fs::create_dir_all(dir)?;
        for _ in 0..file_count {
            let filename = random_name(&mut rng, name_len, ext);
            let path = dir.join(filename);
            let content = random_content(&mut rng, file_size);
            fs::write(path, content)?;
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.just_strings {
        handle_print_strings(cli.file_count, cli.name_len);
        return Ok(());
    }

    handle_generate_files(
        &cli.dirs,
        cli.file_count,
        cli.name_len,
        cli.file_size,
        cli.ext.as_deref(),
        cli.force_create,
    )
}
