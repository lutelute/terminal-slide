use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};

use crate::cli::{ExportFormat, PresentationFormat};

/// Exports a presentation file to the specified format.
pub fn export(
    input: &Path,
    input_format: PresentationFormat,
    export_format: ExportFormat,
    output: Option<&str>,
) -> Result<()> {
    // Validate: md → md is a no-op
    if input_format == PresentationFormat::Markdown && export_format == ExportFormat::Md {
        bail!("Input is already Markdown. No conversion needed.");
    }

    let output_path = match output {
        Some(p) => PathBuf::from(p),
        None => default_output_path(input, export_format),
    };

    match (input_format, export_format) {
        // .md → .pdf (pandoc beamer)
        (PresentationFormat::Markdown, ExportFormat::Pdf) => {
            require_pandoc()?;
            let input_str = preprocess_md_for_pandoc(input)?;
            let tmp = temp_path(input, ".pandoc-tmp.md");
            std::fs::write(&tmp, &input_str)
                .with_context(|| format!("Failed to write temp file: {}", tmp.display()))?;
            let result = run_pandoc(&[
                tmp.to_str().unwrap(),
                "-t",
                "beamer",
                "-o",
                output_path.to_str().unwrap(),
            ]);
            let _ = std::fs::remove_file(&tmp);
            result?;
        }

        // .md → .pptx (pandoc)
        (PresentationFormat::Markdown, ExportFormat::Pptx) => {
            require_pandoc()?;
            let input_str = preprocess_md_for_pandoc(input)?;
            let tmp = temp_path(input, ".pandoc-tmp.md");
            std::fs::write(&tmp, &input_str)
                .with_context(|| format!("Failed to write temp file: {}", tmp.display()))?;
            let result = run_pandoc(&[
                tmp.to_str().unwrap(),
                "-o",
                output_path.to_str().unwrap(),
            ]);
            let _ = std::fs::remove_file(&tmp);
            result?;
        }

        // .html → .pdf (headless Chrome with print-friendly temp file)
        (PresentationFormat::Html, ExportFormat::Pdf) => {
            let chrome = find_chrome()?;
            let tmp_html = prepare_html_for_print(input)?;
            let result = run_chrome_pdf(&chrome, &tmp_html, &output_path);
            let _ = std::fs::remove_file(&tmp_html);
            result?;
        }

        // .html → .md (pandoc)
        (PresentationFormat::Html, ExportFormat::Md) => {
            require_pandoc()?;
            run_pandoc(&[
                input.to_str().unwrap(),
                "-t",
                "markdown",
                "-o",
                output_path.to_str().unwrap(),
            ])?;
        }

        // .html → .pptx (pandoc html→md→pptx)
        (PresentationFormat::Html, ExportFormat::Pptx) => {
            require_pandoc()?;
            let tmp_md = temp_path(input, ".pandoc-tmp.md");
            run_pandoc(&[
                input.to_str().unwrap(),
                "-t",
                "markdown",
                "-o",
                tmp_md.to_str().unwrap(),
            ])?;
            let result = run_pandoc(&[
                tmp_md.to_str().unwrap(),
                "-o",
                output_path.to_str().unwrap(),
            ]);
            let _ = std::fs::remove_file(&tmp_md);
            result?;
        }

        (PresentationFormat::Markdown, ExportFormat::Md) => {
            unreachable!("Already handled above");
        }
    }

    // Verify output was created
    if !output_path.exists() {
        bail!("Export failed: output file was not created");
    }

    let size = std::fs::metadata(&output_path)
        .map(|m| m.len())
        .unwrap_or(0);
    println!(
        "Exported: {} ({:.1} KB)",
        output_path.display(),
        size as f64 / 1024.0
    );
    Ok(())
}

/// Preprocess markdown for pandoc: ensure `---` slide separators have blank lines
/// around them and add YAML header for beamer/pptx compatibility.
fn preprocess_md_for_pandoc(input: &Path) -> Result<String> {
    let content = std::fs::read_to_string(input)
        .with_context(|| format!("Failed to read: {}", input.display()))?;

    let mut output = String::new();
    let mut in_code_block = false;

    // Extract title from first H1
    let title = content
        .lines()
        .find(|l| l.starts_with("# "))
        .map(|l| l.trim_start_matches("# ").trim())
        .unwrap_or("Presentation");

    // Add YAML front matter for pandoc
    output.push_str("---\n");
    output.push_str(&format!("title: \"{title}\"\n"));
    output.push_str("---\n\n");

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_code_block = !in_code_block;
        }

        // Ensure --- slide separators are properly formatted for pandoc
        if !in_code_block && trimmed == "---" {
            if !output.ends_with("\n\n") {
                output.push('\n');
            }
            output.push_str("---\n\n");
        } else {
            output.push_str(line);
            output.push('\n');
        }
    }

    Ok(output)
}

/// Creates a print-friendly version of an HTML presentation.
/// Injects CSS to make all slides visible with page breaks between them.
fn prepare_html_for_print(input: &Path) -> Result<PathBuf> {
    let html = std::fs::read_to_string(input)
        .with_context(|| format!("Failed to read: {}", input.display()))?;

    let print_css = r#"<style>
/* Print overrides: show all slides, one per page */
.slide {
  display: flex !important;
  position: relative !important;
  page-break-after: always;
  break-after: page;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}
.slide:last-of-type { page-break-after: auto; }
/* Hide navigation UI */
.progress, .nav-hint, ._ts-toolbar, ._ts-jump, ._ts-gallery-overlay, ._ts-export-menu { display: none !important; }
/* Disable animations */
* { animation: none !important; transition: none !important; }
body { overflow: visible !important; }
@page { size: landscape; margin: 0; }
</style>"#;

    let modified = if let Some(pos) = html.find("</head>") {
        format!("{}{}{}", &html[..pos], print_css, &html[pos..])
    } else {
        format!("{print_css}{html}")
    };

    let tmp = temp_path(input, ".print-tmp.html");
    std::fs::write(&tmp, &modified)
        .with_context(|| format!("Failed to write temp file: {}", tmp.display()))?;
    Ok(tmp)
}

fn default_output_path(input: &Path, format: ExportFormat) -> PathBuf {
    let stem = input.file_stem().unwrap_or_default().to_string_lossy();
    let ext = match format {
        ExportFormat::Pdf => "pdf",
        ExportFormat::Pptx => "pptx",
        ExportFormat::Md => "md",
    };
    let candidate = input.with_file_name(format!("{stem}.{ext}"));
    // Avoid overwriting the input file or any existing file
    if candidate == input || candidate.exists() {
        input.with_file_name(format!("{stem}_export.{ext}"))
    } else {
        candidate
    }
}

fn temp_path(input: &Path, suffix: &str) -> PathBuf {
    let stem = input.file_stem().unwrap_or_default().to_string_lossy();
    input.with_file_name(format!("{stem}{suffix}"))
}

fn require_pandoc() -> Result<()> {
    match Command::new("pandoc").arg("--version").output() {
        Ok(output) if output.status.success() => Ok(()),
        _ => bail!(
            "pandoc is required but not found.\n\
             Install: https://pandoc.org/installing.html\n\
             macOS:   brew install pandoc"
        ),
    }
}

fn run_pandoc(args: &[&str]) -> Result<()> {
    let output = Command::new("pandoc")
        .args(args)
        .output()
        .context("Failed to run pandoc")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("pandoc failed:\n{stderr}");
    }
    Ok(())
}

fn find_chrome() -> Result<String> {
    // Check CHROME_PATH env var first
    if let Ok(path) = std::env::var("CHROME_PATH") {
        if Path::new(&path).exists() {
            return Ok(path);
        }
    }

    let candidates = if cfg!(target_os = "macos") {
        vec![
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
            "/Applications/Brave Browser.app/Contents/MacOS/Brave Browser",
        ]
    } else {
        vec![
            "google-chrome",
            "google-chrome-stable",
            "chromium",
            "chromium-browser",
        ]
    };

    for candidate in &candidates {
        if cfg!(target_os = "macos") {
            if Path::new(candidate).exists() {
                return Ok(candidate.to_string());
            }
        } else {
            // On Linux, check if command exists via `which`
            if let Ok(output) = Command::new("which").arg(candidate).output() {
                if output.status.success() {
                    return Ok(candidate.to_string());
                }
            }
        }
    }

    bail!(
        "Chrome/Chromium is required for HTML→PDF export but was not found.\n\
         Set CHROME_PATH environment variable or install Chrome/Chromium.\n\
         macOS:   brew install --cask google-chrome"
    )
}

fn run_chrome_pdf(chrome: &str, input: &Path, output: &Path) -> Result<()> {
    let input_abs = input
        .canonicalize()
        .with_context(|| format!("Cannot resolve path: {}", input.display()))?;
    let input_url = format!("file://{}", input_abs.display());

    let output_arg = format!("--print-to-pdf={}", output.display());

    let result = Command::new(chrome)
        .args([
            "--headless",
            "--disable-gpu",
            "--no-sandbox",
            "--run-all-compositor-stages-before-draw",
            "--virtual-time-budget=3000",
            &output_arg,
            &input_url,
        ])
        .output()
        .context("Failed to run Chrome")?;

    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        bail!("Chrome PDF export failed:\n{stderr}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_output_path() {
        let input = Path::new("slides.md");
        assert_eq!(
            default_output_path(input, ExportFormat::Pdf),
            PathBuf::from("slides.pdf")
        );
        assert_eq!(
            default_output_path(input, ExportFormat::Pptx),
            PathBuf::from("slides.pptx")
        );

        let html_input = Path::new("presentation.html");
        assert_eq!(
            default_output_path(html_input, ExportFormat::Md),
            PathBuf::from("presentation.md")
        );
    }

    #[test]
    fn test_default_output_path_with_directory() {
        let input = Path::new("/path/to/slides.md");
        assert_eq!(
            default_output_path(input, ExportFormat::Pdf),
            PathBuf::from("/path/to/slides.pdf")
        );
    }
}
