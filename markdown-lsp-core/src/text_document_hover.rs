use lsp_server::Request;
use lsp_types::{Hover, HoverContents, HoverParams, MarkedString};
use std::error::Error;
use tracing::info;

pub fn handle_text_document_hover(request: Request) -> Result<Hover, Box<dyn Error>> {
    let hover_params: HoverParams = serde_json::from_value(request.params)?;
    info!(?hover_params);
    // TODO: actual implementation
    Ok(Hover {
        contents: HoverContents::Scalar(MarkedString::String(
            "this is just something bro".to_string(),
        )),
        range: None,
    })
}
