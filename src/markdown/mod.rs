pub mod parser;
pub mod renderer;

pub use parser::parse_presentation;
pub use renderer::{SlideWidget, SyntaxHighlighter};
