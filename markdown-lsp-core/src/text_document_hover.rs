use lsp_server::Request;
use lsp_types::{Hover, HoverContents, HoverParams, MarkedString};
use std::{error::Error, fs};
use tracing::info;

pub fn handle_text_document_hover(request: Request) -> Result<Hover, Box<dyn Error>> {
    let hover_params: HoverParams = serde_json::from_value(request.params)?;
    info!(?hover_params);

    // read document's content into a string (assuming file is on local fs)
    let file_path = hover_params
        .text_document_position_params
        .text_document
        .uri
        .as_str()
        .replace("file://", "");
    let Ok(document_content) = fs::read_to_string(&file_path) else {
        return Err(format!("Can't open file at '{}'", file_path).into());
    };

    let position = hover_params.text_document_position_params.position;
    let line = position.line as usize;
    let character = position.character as usize;
    let hover_content = handle(document_content, line, character)?;

    Ok(Hover {
        contents: HoverContents::Scalar(MarkedString::String(hover_content)),
        range: None,
    })
}

#[derive(Debug)]
enum HandleTextDocumentHoverError {
    InvalidLineNumber,
    InvalidCharNumber,
}

impl std::fmt::Display for HandleTextDocumentHoverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for HandleTextDocumentHoverError {}

/// Handle textDocument/hover request
///
/// # Arguments
/// - `content`: the full markdown file to handle
/// - `line`: hover params line
/// - `character`: hover params character
///
/// # Return
/// A human readable string that will be displayed in the hover popup
fn handle(
    content: String,
    line: usize,
    character: usize,
) -> Result<String, HandleTextDocumentHoverError> {
    let lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();

    let hovered_line = lines
        .get(line)
        .ok_or(HandleTextDocumentHoverError::InvalidLineNumber)?;

    let hovered_lines_chars: Vec<char> = hovered_line.chars().collect();
    let hovered_char = hovered_lines_chars
        .get(character)
        .ok_or(HandleTextDocumentHoverError::InvalidCharNumber)?;

    let mut hover_content = String::new();
    if let Some(header_level) = header_level(&hovered_line) {
        hover_content.push_str(format!("Header {}\n", header_level).as_str());
    }

    hover_content.push_str(format!("focused character: '{}'", hovered_char).as_str());

    Ok(hover_content)
}

fn header_level(line: &str) -> Option<usize> {
    if !line.starts_with("#") {
        return None;
    }

    let current_len = line.len();
    let r1 = line.replace("#", "");

    if r1.len() == current_len {
        return None;
    }

    if !r1.starts_with(" ") {
        return None;
    }

    Some(current_len - r1.len())
}

#[cfg(test)]
mod tests {
    use crate::text_document_hover::header_level;
    use rstest::rstest;

    use super::handle;

    #[test]
    fn test_handle_h1() {
        let content = "# h1".to_string();
        let line = 0;
        let character = 2;

        let expected = "Header 1\nfocused character: 'h'";
        let actual = handle(content, line, character).unwrap();
        assert_eq!(expected, &actual);
    }

    #[rstest]
    #[case("# hello", Some(1))]
    #[case("## world", Some(2))]
    #[case("### of markdown", Some(3))]
    #[case("hello world", None)]
    #[case("#", None)]
    #[case("#hello", None)]
    fn test_header_level(#[case] line: &str, #[case] expected_header_level: Option<usize>) {
        let actual = header_level(line);
        assert_eq!(expected_header_level, actual);
    }
}
