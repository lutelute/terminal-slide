/// Represents inline text styling for spans within slide elements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextStyle {
    Plain,
    Bold,
    Italic,
    BoldItalic,
    Code,
}

/// A span of text with an associated style.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyledSpan {
    pub text: String,
    pub style: TextStyle,
}

impl StyledSpan {
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: TextStyle::Plain,
        }
    }

    pub fn bold(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: TextStyle::Bold,
        }
    }

    pub fn italic(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: TextStyle::Italic,
        }
    }

    pub fn bold_italic(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: TextStyle::BoldItalic,
        }
    }

    pub fn code(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: TextStyle::Code,
        }
    }
}

/// Elements that compose a single slide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlideElement {
    /// A heading with a level (1-6) and text content.
    Heading(u8, String),
    /// A paragraph composed of styled text spans.
    Paragraph(Vec<StyledSpan>),
    /// A fenced code block with an optional language tag and the code content.
    CodeBlock(Option<String>, String),
    /// An unordered (bullet) list where each item is a list of styled spans.
    BulletList(Vec<Vec<StyledSpan>>),
    /// An ordered (numbered) list where each item is a list of styled spans.
    NumberedList(Vec<Vec<StyledSpan>>),
    /// A horizontal rule separator.
    HorizontalRule,
    /// A blank line for spacing (reserved for future use).
    #[allow(dead_code)]
    BlankLine,
}

/// A single slide in the presentation.
#[derive(Debug, Clone)]
pub struct Slide {
    /// The raw markdown content of this slide (before parsing).
    #[allow(dead_code)]
    pub raw_content: String,
    /// The parsed elements that make up this slide.
    pub elements: Vec<SlideElement>,
}

impl Slide {
    /// Creates a new slide with the given raw content and parsed elements.
    pub fn new(raw_content: impl Into<String>, elements: Vec<SlideElement>) -> Self {
        Self {
            raw_content: raw_content.into(),
            elements,
        }
    }

    /// Creates an empty slide with no content.
    pub fn empty() -> Self {
        Self {
            raw_content: String::new(),
            elements: Vec::new(),
        }
    }

    /// Returns true if the slide has no elements.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

/// A complete slide presentation.
#[derive(Debug, Clone)]
pub struct Presentation {
    /// The slides in this presentation, in order.
    pub slides: Vec<Slide>,
    /// An optional title for the presentation (extracted from the first H1 heading).
    #[allow(dead_code)]
    pub title: Option<String>,
}

impl Presentation {
    /// Creates a new presentation with the given slides and optional title.
    pub fn new(slides: Vec<Slide>, title: Option<String>) -> Self {
        Self { slides, title }
    }

    /// Returns the number of slides in the presentation.
    pub fn slide_count(&self) -> usize {
        self.slides.len()
    }

    /// Returns a reference to the slide at the given index, if it exists.
    pub fn get_slide(&self, index: usize) -> Option<&Slide> {
        self.slides.get(index)
    }

    /// Returns true if the presentation has no slides.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.slides.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_slide() {
        let slide = Slide::new(
            "# Hello\n\nWorld",
            vec![
                SlideElement::Heading(1, "Hello".to_string()),
                SlideElement::Paragraph(vec![StyledSpan::plain("World")]),
            ],
        );

        assert_eq!(slide.raw_content, "# Hello\n\nWorld");
        assert_eq!(slide.elements.len(), 2);
        assert!(!slide.is_empty());
    }

    #[test]
    fn test_create_empty_slide() {
        let slide = Slide::empty();

        assert_eq!(slide.raw_content, "");
        assert!(slide.elements.is_empty());
        assert!(slide.is_empty());
    }

    #[test]
    fn test_create_presentation() {
        let slides = vec![
            Slide::new(
                "# Slide 1",
                vec![SlideElement::Heading(1, "Slide 1".to_string())],
            ),
            Slide::new(
                "# Slide 2",
                vec![SlideElement::Heading(1, "Slide 2".to_string())],
            ),
            Slide::new(
                "# Slide 3",
                vec![SlideElement::Heading(1, "Slide 3".to_string())],
            ),
        ];

        let presentation = Presentation::new(slides, Some("My Talk".to_string()));

        assert_eq!(presentation.slide_count(), 3);
        assert_eq!(presentation.title, Some("My Talk".to_string()));
        assert!(!presentation.is_empty());
    }

    #[test]
    fn test_presentation_slide_count() {
        let empty = Presentation::new(Vec::new(), None);
        assert_eq!(empty.slide_count(), 0);
        assert!(empty.is_empty());

        let single = Presentation::new(vec![Slide::empty()], None);
        assert_eq!(single.slide_count(), 1);
        assert!(!single.is_empty());
    }

    #[test]
    fn test_presentation_get_slide() {
        let slides = vec![
            Slide::new("first", vec![SlideElement::Heading(1, "First".to_string())]),
            Slide::new(
                "second",
                vec![SlideElement::Heading(1, "Second".to_string())],
            ),
        ];

        let presentation = Presentation::new(slides, None);

        assert!(presentation.get_slide(0).is_some());
        assert!(presentation.get_slide(1).is_some());
        assert!(presentation.get_slide(2).is_none());

        let first = presentation.get_slide(0).unwrap();
        assert_eq!(first.raw_content, "first");
    }

    #[test]
    fn test_slide_elements_variants() {
        let elements = vec![
            SlideElement::Heading(1, "Title".to_string()),
            SlideElement::Paragraph(vec![StyledSpan::plain("Hello "), StyledSpan::bold("world")]),
            SlideElement::CodeBlock(Some("rust".to_string()), "fn main() {}".to_string()),
            SlideElement::BulletList(vec![
                vec![StyledSpan::plain("Item 1")],
                vec![StyledSpan::plain("Item 2")],
            ]),
            SlideElement::NumberedList(vec![
                vec![StyledSpan::plain("First")],
                vec![StyledSpan::plain("Second")],
            ]),
            SlideElement::HorizontalRule,
            SlideElement::BlankLine,
        ];

        let slide = Slide::new("raw content", elements.clone());
        assert_eq!(slide.elements.len(), 7);

        // Verify each variant
        assert!(matches!(&slide.elements[0], SlideElement::Heading(1, t) if t == "Title"));
        assert!(matches!(&slide.elements[1], SlideElement::Paragraph(spans) if spans.len() == 2));
        assert!(
            matches!(&slide.elements[2], SlideElement::CodeBlock(Some(lang), code) if lang == "rust" && code == "fn main() {}")
        );
        assert!(matches!(&slide.elements[3], SlideElement::BulletList(items) if items.len() == 2));
        assert!(
            matches!(&slide.elements[4], SlideElement::NumberedList(items) if items.len() == 2)
        );
        assert!(matches!(&slide.elements[5], SlideElement::HorizontalRule));
        assert!(matches!(&slide.elements[6], SlideElement::BlankLine));
    }

    #[test]
    fn test_styled_span_constructors() {
        let plain = StyledSpan::plain("hello");
        assert_eq!(plain.text, "hello");
        assert_eq!(plain.style, TextStyle::Plain);

        let bold = StyledSpan::bold("strong");
        assert_eq!(bold.text, "strong");
        assert_eq!(bold.style, TextStyle::Bold);

        let italic = StyledSpan::italic("emphasis");
        assert_eq!(italic.text, "emphasis");
        assert_eq!(italic.style, TextStyle::Italic);

        let bold_italic = StyledSpan::bold_italic("both");
        assert_eq!(bold_italic.text, "both");
        assert_eq!(bold_italic.style, TextStyle::BoldItalic);

        let code = StyledSpan::code("inline");
        assert_eq!(code.text, "inline");
        assert_eq!(code.style, TextStyle::Code);
    }

    #[test]
    fn test_code_block_without_language() {
        let element = SlideElement::CodeBlock(None, "some code".to_string());
        assert!(matches!(element, SlideElement::CodeBlock(None, _)));
    }

    #[test]
    fn test_presentation_without_title() {
        let presentation = Presentation::new(vec![Slide::empty()], None);
        assert_eq!(presentation.title, None);
        assert_eq!(presentation.slide_count(), 1);
    }
}
