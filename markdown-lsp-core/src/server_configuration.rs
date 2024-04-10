use lsp_types::{HoverProviderCapability, ServerCapabilities};

// TODO: get this information from rust cargo workspace
pub const SERVER_NAME: &str = "markdown-lsp-server";
pub const SERVER_VERSION: &str = "0.1";

pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        position_encoding: None,
        text_document_sync: None,
        selection_range_provider: None,
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        completion_provider: None,
        signature_help_provider: None,
        definition_provider: None,
        type_definition_provider: None,
        implementation_provider: None,
        references_provider: None,
        document_highlight_provider: None,
        document_symbol_provider: None,
        workspace_symbol_provider: None,
        code_action_provider: None,
        code_lens_provider: None,
        document_formatting_provider: None,
        document_range_formatting_provider: None,
        document_on_type_formatting_provider: None,
        rename_provider: None,
        document_link_provider: None,
        color_provider: None,
        folding_range_provider: None,
        declaration_provider: None,
        execute_command_provider: None,
        workspace: None,
        call_hierarchy_provider: None,
        semantic_tokens_provider: None,
        moniker_provider: None,
        linked_editing_range_provider: None,
        inline_value_provider: None,
        inlay_hint_provider: None,
        diagnostic_provider: None,
        experimental: None,
    }
}
