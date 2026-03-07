use std::path::Path;

use anyhow::{bail, Result};
use clap::Parser;

mod app;
mod cli;
mod markdown;
mod slide;
mod tui;

use cli::{Cli, PresentationFormat, detect_format};

fn main() -> Result<()> {
    let args = Cli::parse();

    let path = Path::new(&args.file);
    if !path.exists() {
        bail!("File not found: {}", args.file);
    }

    let format = detect_format(&args.file)?;

    match format {
        PresentationFormat::Markdown => {
            println!("Presenting markdown slides: {}", args.file);
            // TODO: Parse markdown and launch TUI presentation
        }
        PresentationFormat::Html => {
            println!(
                "Serving HTML slides: {} on port {}",
                args.file, args.port
            );
            // TODO: Launch HTTP server and open browser
        }
    }

    Ok(())
}
