use lsp_server::Request;
use lsp_types::{Hover, HoverContents, HoverParams, MarkedString};
use std::{error::Error, fs};
use tracing::info;

pub fn handle_text_document_hover(request: Request) -> Result<Hover, Box<dyn Error>> {
    let hover_params: HoverParams = serde_json::from_value(request.params)?;
    info!(?hover_params);

    // read document's content into a string
    let file_path = hover_params
        .text_document_position_params
        .text_document
        .uri
        .as_str()
        .replace("file://", "");
    let Ok(document_content) = fs::read_to_string(&file_path) else {
        return Err(format!("Can't open file at '{}'", file_path).into());
    };

    // read content at precised position
    let position = hover_params.text_document_position_params.position;
    let line = position.line as usize;
    let character = position.character as usize;

    let content_lines: Vec<String> = document_content
        .lines()
        .map(|line| line.to_string())
        .collect();

    let interesting_line = content_lines.get(line).expect("line must be there");

    let interesting_char: Vec<char> = interesting_line.chars().skip(character).take(1).collect();

    let interesting_char = interesting_char.first().unwrap();

    let is_h1 = interesting_line.starts_with("# ");

    // TODO: actual implementation
    Ok(Hover {
        contents: HoverContents::Scalar(MarkedString::String(format!(
            "Your cursor is on this char '{}'\nIs this a header 1 ? {}",
            interesting_char, is_h1
        ))),
        range: None,
    })
}
