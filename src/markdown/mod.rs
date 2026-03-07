pub mod parser;
pub mod renderer;

pub use parser::{parse_presentation, parse_slide, split_slides};
pub use renderer::{SlideWidget, SyntaxHighlighter};
