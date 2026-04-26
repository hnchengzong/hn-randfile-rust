mod cli;
mod generator;
mod random;

use cli::Cli;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.just_strings {
        generator::handle_print_strings(cli.file_count, cli.name_len);
        return Ok(());
    }

    generator::handle_generate_files(
        &cli.dirs,
        cli.file_count,
        cli.name_len,
        cli.file_size,
        cli.ext.as_deref(),
        cli.force_create,
    )
}
