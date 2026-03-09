use std::path::Path;

use anyhow::{bail, Result};
use clap::{Parser, ValueEnum};

/// Export format for slide presentations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ExportFormat {
    Pdf,
    Pptx,
    Md,
    Html,
}

/// Terminal-based slide presentation tool supporting Markdown and HTML.
///
/// Renders .md files as interactive TUI presentations in the terminal.
/// Serves .html files via a local HTTP server and opens them in the browser.
#[derive(Parser, Debug)]
#[command(name = "terminal-slide", version, about)]
pub struct Cli {
    /// Path to the presentation file (.md, .html, or .htm)
    pub file: String,

    /// Port for the local HTTP server (used for HTML presentations)
    #[arg(long, default_value_t = 8234)]
    pub port: u16,

    /// Export to a file instead of presenting (pdf, pptx, md)
    #[arg(long, value_enum)]
    pub export: Option<ExportFormat>,

    /// Output file path for export (defaults to input filename with new extension)
    #[arg(short, long)]
    pub output: Option<String>,
}

/// The detected presentation format based on file extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationFormat {
    Markdown,
    Html,
}

/// Detects the presentation format from the file extension.
///
/// Returns `Ok(PresentationFormat)` for supported extensions (.md, .html, .htm),
/// or an error for unsupported extensions.
pub fn detect_format(path: &str) -> Result<PresentationFormat> {
    let extension = Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());

    match extension.as_deref() {
        Some("md") => Ok(PresentationFormat::Markdown),
        Some("html") | Some("htm") => Ok(PresentationFormat::Html),
        Some(ext) => bail!("Unsupported file format: .{ext}\nSupported formats: .md, .html, .htm"),
        None => bail!("File has no extension.\nSupported formats: .md, .html, .htm"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_detection() {
        assert_eq!(
            detect_format("slides.md").unwrap(),
            PresentationFormat::Markdown
        );
        assert_eq!(
            detect_format("presentation.html").unwrap(),
            PresentationFormat::Html
        );
        assert_eq!(
            detect_format("slides.htm").unwrap(),
            PresentationFormat::Html
        );
    }

    #[test]
    fn test_format_detection_case_insensitive() {
        assert_eq!(
            detect_format("slides.MD").unwrap(),
            PresentationFormat::Markdown
        );
        assert_eq!(
            detect_format("slides.HTML").unwrap(),
            PresentationFormat::Html
        );
        assert_eq!(
            detect_format("slides.Htm").unwrap(),
            PresentationFormat::Html
        );
    }

    #[test]
    fn test_format_detection_with_path() {
        assert_eq!(
            detect_format("/path/to/slides.md").unwrap(),
            PresentationFormat::Markdown
        );
        assert_eq!(
            detect_format("./relative/path/slides.html").unwrap(),
            PresentationFormat::Html
        );
    }

    #[test]
    fn test_invalid_extension() {
        let result = detect_format("presentation.pptx");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Unsupported file format: .pptx"));
        assert!(err.contains("Supported formats"));

        let result = detect_format("document.pdf");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Unsupported file format: .pdf"));

        let result = detect_format("image.png");
        assert!(result.is_err());

        let result = detect_format("slides.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_no_extension() {
        let result = detect_format("README");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("no extension"));
    }
}
