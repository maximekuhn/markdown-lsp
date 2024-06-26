use std::error::Error;
use tracing::info;

mod logger;

fn main() -> Result<(), Box<dyn Error>> {
    let _guard = logger::init_logger();
    info!("markdown-lsp started !");

    let mut lsp_context = markdown_lsp_core::LspContext::default();
    lsp_context.init()?;
    lsp_context.main_loop()?;
    lsp_context.shutdown_gracefully()?;

    Ok(())
}
