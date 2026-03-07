use std::fs;
use std::path::Path;

use anyhow::{bail, Context, Result};
use tiny_http::{Header, Request, Response, Server};

/// Serves an HTML presentation file via a local HTTP server and opens the default browser.
///
/// Starts a tiny_http server on `localhost:port`, serving the HTML file at the `/` route
/// and any co-located assets (CSS, JS, images) from the same directory. If the requested
/// port is occupied, tries up to 10 subsequent ports before failing.
///
/// The server runs until the process is terminated (Ctrl+C).
pub fn serve_html(file_path: &Path, port: u16) -> Result<()> {
    let file_path = file_path
        .canonicalize()
        .with_context(|| format!("Cannot resolve path: {}", file_path.display()))?;

    let base_dir = file_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine parent directory"))?
        .to_path_buf();

    let file_name = file_path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine file name"))?
        .to_string_lossy()
        .to_string();

    // Try binding to the requested port, falling back to port+1..port+10
    let (server, actual_port) = try_bind_server(port)?;

    let url = format!("http://localhost:{actual_port}");

    println!("Serving: {}", file_path.display());
    println!("Server running at: {url}");
    println!("Press Ctrl+C to stop");

    // Open the default browser
    if let Err(e) = open::that(&url) {
        eprintln!("Warning: Could not open browser: {e}");
    }

    // Handle incoming requests until the process is terminated
    for request in server.incoming_requests() {
        handle_request(request, &base_dir, &file_name);
    }

    Ok(())
}

/// Attempts to bind a server to the given port, trying up to 10 subsequent ports
/// if the requested port is occupied.
fn try_bind_server(port: u16) -> Result<(Server, u16)> {
    for offset in 0..=10u16 {
        let try_port = match port.checked_add(offset) {
            Some(p) => p,
            None => break,
        };

        let addr = format!("127.0.0.1:{try_port}");
        match Server::http(&addr) {
            Ok(server) => {
                if offset > 0 {
                    eprintln!("Port {port} is in use, using port {try_port} instead");
                }
                return Ok((server, try_port));
            }
            Err(_) if offset < 10 => continue,
            Err(e) => {
                bail!("Could not bind to any port in range {port}-{try_port}: {e}");
            }
        }
    }

    bail!(
        "Could not bind to any port in range {}-{}",
        port,
        port.saturating_add(10)
    )
}

/// Handles a single HTTP request by serving files from the base directory.
fn handle_request(request: Request, base_dir: &Path, index_file: &str) {
    // Strip query string before decoding the path
    let raw_url = request.url().to_string();
    let path_portion = raw_url.split('?').next().unwrap_or(&raw_url);
    let url_path = percent_decode(path_portion);

    let relative = if url_path == "/" {
        index_file.to_string()
    } else {
        url_path.trim_start_matches('/').to_string()
    };

    let file_path = base_dir.join(&relative);

    // Security: ensure the resolved path stays within the base directory
    let canonical = match file_path.canonicalize() {
        Ok(p) if p.starts_with(base_dir) => p,
        _ => {
            let response = Response::from_string("404 Not Found").with_status_code(404);
            let _ = request.respond(response);
            return;
        }
    };

    match fs::read(&canonical) {
        Ok(content) => {
            let mime = guess_mime_type(&canonical);
            let header =
                Header::from_bytes("Content-Type", mime).expect("valid Content-Type header");
            let response = Response::from_data(content).with_header(header);
            let _ = request.respond(response);
        }
        Err(_) => {
            let response = Response::from_string("404 Not Found").with_status_code(404);
            let _ = request.respond(response);
        }
    }
}

/// Guesses the MIME type of a file based on its extension.
fn guess_mime_type(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .as_deref()
    {
        Some("html") | Some("htm") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") | Some("mjs") => "application/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("webp") => "image/webp",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        Some("otf") => "font/otf",
        Some("mp4") => "video/mp4",
        Some("webm") => "video/webm",
        Some("xml") => "application/xml",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

/// Decodes percent-encoded characters in a URL path.
fn percent_decode(input: &str) -> String {
    let mut result = Vec::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(decoded) =
                u8::from_str_radix(std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or(""), 16)
            {
                result.push(decoded);
                i += 3;
                continue;
            }
        }
        result.push(bytes[i]);
        i += 1;
    }

    String::from_utf8_lossy(&result).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_mime_type_html() {
        assert_eq!(
            guess_mime_type(Path::new("index.html")),
            "text/html; charset=utf-8"
        );
        assert_eq!(
            guess_mime_type(Path::new("page.htm")),
            "text/html; charset=utf-8"
        );
    }

    #[test]
    fn test_guess_mime_type_css_js() {
        assert_eq!(
            guess_mime_type(Path::new("style.css")),
            "text/css; charset=utf-8"
        );
        assert_eq!(
            guess_mime_type(Path::new("app.js")),
            "application/javascript; charset=utf-8"
        );
        assert_eq!(
            guess_mime_type(Path::new("module.mjs")),
            "application/javascript; charset=utf-8"
        );
    }

    #[test]
    fn test_guess_mime_type_images() {
        assert_eq!(guess_mime_type(Path::new("logo.png")), "image/png");
        assert_eq!(guess_mime_type(Path::new("photo.jpg")), "image/jpeg");
        assert_eq!(guess_mime_type(Path::new("photo.jpeg")), "image/jpeg");
        assert_eq!(guess_mime_type(Path::new("icon.gif")), "image/gif");
        assert_eq!(guess_mime_type(Path::new("graphic.svg")), "image/svg+xml");
        assert_eq!(guess_mime_type(Path::new("image.webp")), "image/webp");
    }

    #[test]
    fn test_guess_mime_type_fonts() {
        assert_eq!(guess_mime_type(Path::new("font.woff")), "font/woff");
        assert_eq!(guess_mime_type(Path::new("font.woff2")), "font/woff2");
        assert_eq!(guess_mime_type(Path::new("font.ttf")), "font/ttf");
    }

    #[test]
    fn test_guess_mime_type_unknown() {
        assert_eq!(
            guess_mime_type(Path::new("data.xyz")),
            "application/octet-stream"
        );
        assert_eq!(
            guess_mime_type(Path::new("noextension")),
            "application/octet-stream"
        );
    }

    #[test]
    fn test_percent_decode_plain() {
        assert_eq!(percent_decode("/path/to/file"), "/path/to/file");
    }

    #[test]
    fn test_percent_decode_spaces() {
        assert_eq!(percent_decode("/my%20file.html"), "/my file.html");
    }

    #[test]
    fn test_percent_decode_special_chars() {
        assert_eq!(percent_decode("/file%2Fname"), "/file/name");
        assert_eq!(percent_decode("%48%65%6C%6C%6F"), "Hello");
    }

    #[test]
    fn test_percent_decode_invalid() {
        // Invalid percent sequences are left as-is
        assert_eq!(percent_decode("/file%ZZ"), "/file%ZZ");
        assert_eq!(percent_decode("/file%2"), "/file%2");
        assert_eq!(percent_decode("/file%"), "/file%");
    }

    #[test]
    #[ignore] // Requires network port binding (not available in sandboxed environments)
    fn test_try_bind_server() {
        let result = try_bind_server(18234);
        assert!(result.is_ok());
        let (_server, port) = result.unwrap();
        assert!(port >= 18234 && port <= 18244);
    }

    #[test]
    #[ignore] // Requires network port binding (not available in sandboxed environments)
    fn test_try_bind_server_fallback() {
        // Bind to a port first, then try binding to the same port
        let (server1, port1) = try_bind_server(18250).unwrap();
        let (server2, port2) = try_bind_server(port1).unwrap();

        // The second server should have picked a different port
        assert_ne!(port1, port2);
        assert!(port2 > port1);
        assert!(port2 <= port1 + 10);

        drop(server1);
        drop(server2);
    }
}
