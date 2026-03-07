use std::mem;

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser as MdParser, Tag, TagEnd};

use crate::slide::{Presentation, Slide, SlideElement, StyledSpan};

/// Splits markdown content into individual slide strings using `---` as separators.
///
/// The `---` separator must appear on its own line (after trimming whitespace).
/// Separators inside fenced code blocks are ignored.
pub fn split_slides(content: &str) -> Vec<String> {
    let mut slides = Vec::new();
    let mut current_slide = String::new();
    let mut in_code_block = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Track fenced code blocks (``` or ~~~)
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_code_block = !in_code_block;
        }

        // Split on `---` only outside code blocks
        if !in_code_block && trimmed == "---" {
            slides.push(current_slide.trim().to_string());
            current_slide = String::new();
        } else {
            if !current_slide.is_empty() {
                current_slide.push('\n');
            }
            current_slide.push_str(line);
        }
    }

    // Push the last slide
    slides.push(current_slide.trim().to_string());

    // Ensure at least one slide
    if slides.is_empty() {
        slides.push(String::new());
    }

    slides
}

/// Converts a `HeadingLevel` to a numeric level (1-6).
fn heading_level_to_u8(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// Creates a styled span based on the current bold/italic state.
fn make_span(text: String, is_bold: bool, is_italic: bool) -> StyledSpan {
    match (is_bold, is_italic) {
        (true, true) => StyledSpan::bold_italic(text),
        (true, false) => StyledSpan::bold(text),
        (false, true) => StyledSpan::italic(text),
        (false, false) => StyledSpan::plain(text),
    }
}

/// Parses a single slide's markdown content into a `Slide` with structured elements.
///
/// Converts pulldown-cmark events into `SlideElement` variants, handling nested
/// inline styles (bold, italic, code) within paragraphs and list items.
pub fn parse_slide(content: &str) -> Slide {
    if content.trim().is_empty() {
        return Slide::empty();
    }

    let parser = MdParser::new(content);
    let mut elements = Vec::new();

    // Inline style tracking
    let mut is_bold = false;
    let mut is_italic = false;

    // Current spans being accumulated for paragraphs/list items
    let mut current_spans: Vec<StyledSpan> = Vec::new();

    // Heading state
    let mut in_heading = false;
    let mut heading_level: u8 = 1;
    let mut heading_text = String::new();

    // Code block state
    let mut in_code_block = false;
    let mut code_block_lang: Option<String> = None;
    let mut code_block_content = String::new();

    // List state
    let mut list_ordered = false;
    let mut list_items: Vec<Vec<StyledSpan>> = Vec::new();
    let mut in_list_item = false;

    for event in parser {
        match event {
            // Headings
            Event::Start(Tag::Heading { level, .. }) => {
                in_heading = true;
                heading_level = heading_level_to_u8(level);
                heading_text.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                elements.push(SlideElement::Heading(heading_level, heading_text.clone()));
                heading_text.clear();
            }

            // Paragraphs
            Event::Start(Tag::Paragraph) => {
                if !in_list_item {
                    current_spans.clear();
                } else if !current_spans.is_empty() {
                    // Add space between multiple paragraphs within a list item
                    current_spans.push(StyledSpan::plain(" "));
                }
            }
            Event::End(TagEnd::Paragraph) => {
                if !in_list_item && !current_spans.is_empty() {
                    elements.push(SlideElement::Paragraph(mem::take(&mut current_spans)));
                }
            }

            // Code blocks
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_block_lang = match kind {
                    CodeBlockKind::Fenced(lang) => {
                        let lang_str = lang.to_string();
                        if lang_str.is_empty() {
                            None
                        } else {
                            Some(lang_str)
                        }
                    }
                    CodeBlockKind::Indented => None,
                };
                code_block_content.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                elements.push(SlideElement::CodeBlock(
                    code_block_lang.take(),
                    code_block_content.trim_end().to_string(),
                ));
                code_block_content.clear();
            }

            // Lists
            Event::Start(Tag::List(start_number)) => {
                list_ordered = start_number.is_some();
                list_items.clear();
            }
            Event::End(TagEnd::List(_)) => {
                let items = mem::take(&mut list_items);
                if list_ordered {
                    elements.push(SlideElement::NumberedList(items));
                } else {
                    elements.push(SlideElement::BulletList(items));
                }
            }

            // List items
            Event::Start(Tag::Item) => {
                in_list_item = true;
                current_spans.clear();
            }
            Event::End(TagEnd::Item) => {
                in_list_item = false;
                list_items.push(mem::take(&mut current_spans));
            }

            // Inline styles
            Event::Start(Tag::Strong) => {
                is_bold = true;
            }
            Event::End(TagEnd::Strong) => {
                is_bold = false;
            }
            Event::Start(Tag::Emphasis) => {
                is_italic = true;
            }
            Event::End(TagEnd::Emphasis) => {
                is_italic = false;
            }

            // Text content
            Event::Text(text) => {
                let text_str = text.to_string();
                if in_code_block {
                    code_block_content.push_str(&text_str);
                } else if in_heading {
                    heading_text.push_str(&text_str);
                } else {
                    current_spans.push(make_span(text_str, is_bold, is_italic));
                }
            }

            // Inline code
            Event::Code(text) => {
                if in_heading {
                    heading_text.push_str(&text);
                } else {
                    current_spans.push(StyledSpan::code(text.to_string()));
                }
            }

            // Line breaks
            Event::SoftBreak => {
                if in_heading {
                    heading_text.push(' ');
                } else if !in_code_block {
                    current_spans.push(StyledSpan::plain(" "));
                }
            }
            Event::HardBreak => {
                if in_heading {
                    heading_text.push(' ');
                } else if !in_code_block {
                    current_spans.push(StyledSpan::plain("\n"));
                }
            }

            // Horizontal rules
            Event::Rule => {
                elements.push(SlideElement::HorizontalRule);
            }

            // Skip unsupported events (tables, footnotes, images, etc.)
            _ => {}
        }
    }

    Slide::new(content.to_string(), elements)
}

/// Parses a complete markdown presentation into a `Presentation`.
///
/// Splits the content by `---` separators, parses each section as a slide,
/// and extracts the title from the first H1 heading if present.
pub fn parse_presentation(content: &str) -> Presentation {
    let slide_contents = split_slides(content);
    let mut slides: Vec<Slide> = slide_contents.iter().map(|s| parse_slide(s)).collect();

    // Ensure at least one slide
    if slides.is_empty() {
        slides.push(Slide::empty());
    }

    // Extract title from the first H1 heading in the first slide
    let title = slides.first().and_then(|slide| {
        slide.elements.iter().find_map(|el| {
            if let SlideElement::Heading(1, text) = el {
                Some(text.clone())
            } else {
                None
            }
        })
    });

    Presentation::new(slides, title)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slide::TextStyle;

    // --- split_slides tests ---

    #[test]
    fn test_split_slides() {
        let content = "# Slide 1\n\nContent\n\n---\n\n# Slide 2\n\nMore content\n\n---\n\n# Slide 3";
        let slides = split_slides(content);
        assert_eq!(slides.len(), 3);
        assert!(slides[0].contains("Slide 1"));
        assert!(slides[1].contains("Slide 2"));
        assert!(slides[2].contains("Slide 3"));
    }

    #[test]
    fn test_split_slides_preserves_content() {
        let content = "# Title\n\nParagraph text\n\n---\n\n## Second\n\n- Item";
        let slides = split_slides(content);
        assert_eq!(slides.len(), 2);
        assert_eq!(slides[0], "# Title\n\nParagraph text");
        assert_eq!(slides[1], "## Second\n\n- Item");
    }

    #[test]
    fn test_split_slides_ignores_separator_in_code_block() {
        let content = "# Slide 1\n\n```\n---\n```\n\n---\n\n# Slide 2";
        let slides = split_slides(content);
        assert_eq!(slides.len(), 2);
        assert!(slides[0].contains("Slide 1"));
        assert!(slides[0].contains("---")); // The --- inside code block is preserved
        assert!(slides[1].contains("Slide 2"));
    }

    #[test]
    fn test_split_slides_ignores_separator_in_fenced_tilde_block() {
        let content = "# Slide 1\n\n~~~\n---\n~~~\n\n---\n\n# Slide 2";
        let slides = split_slides(content);
        assert_eq!(slides.len(), 2);
    }

    #[test]
    fn test_multiple_consecutive_separators() {
        let content = "Slide 1\n\n---\n\n---\n\nSlide 3";
        let slides = split_slides(content);
        assert_eq!(slides.len(), 3);
        assert_eq!(slides[0], "Slide 1");
        assert!(slides[1].is_empty()); // Empty slide between separators
        assert_eq!(slides[2], "Slide 3");
    }

    // --- empty / single slide tests ---

    #[test]
    fn test_empty_file() {
        let slides = split_slides("");
        assert_eq!(slides.len(), 1);
        assert!(slides[0].is_empty());

        let presentation = parse_presentation("");
        assert_eq!(presentation.slide_count(), 1);
        assert!(presentation.get_slide(0).unwrap().is_empty());
    }

    #[test]
    fn test_single_slide() {
        let content = "# Only Slide\n\nSome content here";
        let slides = split_slides(content);
        assert_eq!(slides.len(), 1);
        assert!(slides[0].contains("Only Slide"));

        let presentation = parse_presentation(content);
        assert_eq!(presentation.slide_count(), 1);
        assert_eq!(presentation.title, Some("Only Slide".to_string()));
    }

    #[test]
    fn test_parse_empty_slide() {
        let slide = parse_slide("");
        assert!(slide.is_empty());
        assert!(slide.elements.is_empty());
    }

    // --- heading tests ---

    #[test]
    fn test_parse_headings() {
        let content = "# Heading 1\n\n## Heading 2\n\n### Heading 3\n\n#### Heading 4\n\n##### Heading 5\n\n###### Heading 6";
        let slide = parse_slide(content);

        let headings: Vec<_> = slide
            .elements
            .iter()
            .filter_map(|el| {
                if let SlideElement::Heading(level, text) = el {
                    Some((*level, text.as_str()))
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(headings.len(), 6);
        assert_eq!(headings[0], (1, "Heading 1"));
        assert_eq!(headings[1], (2, "Heading 2"));
        assert_eq!(headings[2], (3, "Heading 3"));
        assert_eq!(headings[3], (4, "Heading 4"));
        assert_eq!(headings[4], (5, "Heading 5"));
        assert_eq!(headings[5], (6, "Heading 6"));
    }

    // --- code block tests ---

    #[test]
    fn test_parse_code_block() {
        let content = "```rust\nfn main() {\n    println!(\"hello\");\n}\n```";
        let slide = parse_slide(content);

        let code_blocks: Vec<_> = slide
            .elements
            .iter()
            .filter_map(|el| {
                if let SlideElement::CodeBlock(lang, code) = el {
                    Some((lang.as_deref(), code.as_str()))
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(code_blocks.len(), 1);
        assert_eq!(code_blocks[0].0, Some("rust"));
        assert!(code_blocks[0].1.contains("fn main()"));
        assert!(code_blocks[0].1.contains("println!"));
    }

    #[test]
    fn test_parse_code_block_without_language() {
        let content = "```\nsome code\n```";
        let slide = parse_slide(content);

        let code_block = slide
            .elements
            .iter()
            .find_map(|el| {
                if let SlideElement::CodeBlock(lang, code) = el {
                    Some((lang.clone(), code.clone()))
                } else {
                    None
                }
            })
            .expect("should have a code block");

        assert_eq!(code_block.0, None);
        assert_eq!(code_block.1, "some code");
    }

    // --- list tests ---

    #[test]
    fn test_parse_lists() {
        // Bullet list
        let bullet_content = "- Item 1\n- Item 2\n- Item 3";
        let slide = parse_slide(bullet_content);

        let bullet_lists: Vec<_> = slide
            .elements
            .iter()
            .filter_map(|el| {
                if let SlideElement::BulletList(items) = el {
                    Some(items)
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(bullet_lists.len(), 1);
        assert_eq!(bullet_lists[0].len(), 3);
        assert_eq!(bullet_lists[0][0][0].text, "Item 1");
        assert_eq!(bullet_lists[0][1][0].text, "Item 2");
        assert_eq!(bullet_lists[0][2][0].text, "Item 3");

        // Numbered list
        let numbered_content = "1. First\n2. Second\n3. Third";
        let slide = parse_slide(numbered_content);

        let numbered_lists: Vec<_> = slide
            .elements
            .iter()
            .filter_map(|el| {
                if let SlideElement::NumberedList(items) = el {
                    Some(items)
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(numbered_lists.len(), 1);
        assert_eq!(numbered_lists[0].len(), 3);
        assert_eq!(numbered_lists[0][0][0].text, "First");
        assert_eq!(numbered_lists[0][1][0].text, "Second");
        assert_eq!(numbered_lists[0][2][0].text, "Third");
    }

    // --- inline style tests ---

    #[test]
    fn test_parse_bold_and_italic() {
        let content = "**bold** and *italic* and ***both***";
        let slide = parse_slide(content);

        let paragraph = slide
            .elements
            .iter()
            .find_map(|el| {
                if let SlideElement::Paragraph(spans) = el {
                    Some(spans)
                } else {
                    None
                }
            })
            .expect("should have a paragraph");

        let has_bold = paragraph
            .iter()
            .any(|s| s.style == TextStyle::Bold && s.text == "bold");
        assert!(has_bold, "should contain bold text");

        let has_italic = paragraph
            .iter()
            .any(|s| s.style == TextStyle::Italic && s.text == "italic");
        assert!(has_italic, "should contain italic text");

        let has_bold_italic = paragraph
            .iter()
            .any(|s| s.style == TextStyle::BoldItalic && s.text == "both");
        assert!(has_bold_italic, "should contain bold-italic text");
    }

    #[test]
    fn test_parse_inline_code() {
        let content = "Use `println!()` to print";
        let slide = parse_slide(content);

        let paragraph = slide
            .elements
            .iter()
            .find_map(|el| {
                if let SlideElement::Paragraph(spans) = el {
                    Some(spans)
                } else {
                    None
                }
            })
            .expect("should have a paragraph");

        let has_code = paragraph
            .iter()
            .any(|s| s.style == TextStyle::Code && s.text == "println!()");
        assert!(has_code, "should contain inline code");
    }

    // --- nested element tests ---

    #[test]
    fn test_nested_bold_in_list() {
        let content = "- Normal and **bold** item\n- Another item";
        let slide = parse_slide(content);

        let list = slide
            .elements
            .iter()
            .find_map(|el| {
                if let SlideElement::BulletList(items) = el {
                    Some(items)
                } else {
                    None
                }
            })
            .expect("should have a bullet list");

        assert_eq!(list.len(), 2);

        // First item should have both plain and bold spans
        let first_item = &list[0];
        let has_plain = first_item
            .iter()
            .any(|s| s.style == TextStyle::Plain && s.text.contains("Normal"));
        assert!(has_plain, "list item should contain plain text");

        let has_bold_in_item = first_item
            .iter()
            .any(|s| s.style == TextStyle::Bold && s.text == "bold");
        assert!(has_bold_in_item, "list item should contain bold span");
    }

    #[test]
    fn test_code_inside_paragraph() {
        let content = "Run `cargo build` to compile the project";
        let slide = parse_slide(content);

        let paragraph = slide
            .elements
            .iter()
            .find_map(|el| {
                if let SlideElement::Paragraph(spans) = el {
                    Some(spans)
                } else {
                    None
                }
            })
            .expect("should have a paragraph");

        let styles: Vec<_> = paragraph.iter().map(|s| &s.style).collect();
        assert!(
            styles.contains(&&TextStyle::Plain),
            "should have plain text"
        );
        assert!(
            styles.contains(&&TextStyle::Code),
            "should have inline code"
        );
    }

    // --- horizontal rule tests ---

    #[test]
    fn test_parse_horizontal_rule() {
        // Use *** since --- is the slide separator
        let content = "Before\n\n***\n\nAfter";
        let slide = parse_slide(content);

        let has_rule = slide
            .elements
            .iter()
            .any(|el| matches!(el, SlideElement::HorizontalRule));
        assert!(has_rule, "should contain horizontal rule");
    }

    // --- presentation-level tests ---

    #[test]
    fn test_parse_presentation_extracts_title() {
        let content = "# My Talk\n\nIntroduction\n\n---\n\n## Details\n\nContent here";
        let presentation = parse_presentation(content);
        assert_eq!(presentation.slide_count(), 2);
        assert_eq!(presentation.title, Some("My Talk".to_string()));
    }

    #[test]
    fn test_parse_presentation_no_title() {
        let content = "## Not H1\n\nContent\n\n---\n\nMore content";
        let presentation = parse_presentation(content);
        assert_eq!(presentation.slide_count(), 2);
        assert_eq!(presentation.title, None);
    }

    #[test]
    fn test_parse_presentation_multiple_slides() {
        let content = "# Slide 1\n\n---\n\n# Slide 2\n\n---\n\n# Slide 3\n\n---\n\n# Slide 4";
        let presentation = parse_presentation(content);
        assert_eq!(presentation.slide_count(), 4);
        assert_eq!(presentation.title, Some("Slide 1".to_string()));

        for i in 0..4 {
            let slide = presentation.get_slide(i).unwrap();
            assert!(!slide.is_empty());
        }
    }
}
