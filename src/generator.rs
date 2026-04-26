use crate::random::{random_content, random_name};
use anyhow::Result;
use rand::RngCore;
use std::fs;
use std::path::PathBuf;

pub fn confirm(prompt: &str, force: bool) -> Result<bool> {
    if force {
        return Ok(true);
    }
    println!("{} (y/n)", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().eq_ignore_ascii_case("y"))
}

pub fn handle_print_strings(file_count: usize, name_len: usize) {
    let mut rng = rand::rng();
    for _ in 0..file_count {
        let s: String = rand_distr::Alphanumeric
            .sample_iter(&mut rng)
            .take(name_len)
            .map(char::from)
            .collect();
        println!("{}", s);
    }
}

pub fn handle_generate_files(
    dirs: &[PathBuf],
    file_count: usize,
    name_len: usize,
    file_size: usize,
    ext: Option<&str>,
    force_create: bool,
) -> Result<()> {
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