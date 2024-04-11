use lsp_server::{Connection, IoThreads, Message, Request, Response};
use lsp_types::{InitializeResult, ServerInfo};
use std::error::Error;
use text_document_hover::handle_text_document_hover;
use tracing::{error, info};

mod server_configuration;
mod text_document_hover;

pub struct LspContext {
    connection: Connection,
    _io_threads: IoThreads,
}

impl Default for LspContext {
    fn default() -> Self {
        let (connection, io_threads) = Connection::stdio();
        Self {
            connection,
            _io_threads: io_threads,
        }
    }
}

impl LspContext {
    pub fn init(&self) -> Result<(), Box<dyn Error>> {
        // XXX are we sure the first request is always `initialize` ?
        // https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#initialize
        let (req_id, _client_init_params) = self.connection.initialize_start()?;
        info!(
            lsp_event = "received client initialization request",
            ?req_id
        );

        let server_capabilities = server_configuration::server_capabilities();
        let initialize_result = InitializeResult {
            capabilities: server_capabilities,
            server_info: Some(ServerInfo {
                name: server_configuration::SERVER_NAME.to_string(),
                version: Some(server_configuration::SERVER_VERSION.to_string()),
            }),
        };
        let init_res_json = serde_json::json!(initialize_result);
        self.connection.initialize_finish(req_id, init_res_json)?;

        info!(lsp_event = "sent initialization response");

        Ok(())
    }

    pub fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        while let Ok(incoming_msg) = self.connection.receiver.recv() {
            info!(?incoming_msg);
            match incoming_msg {
                Message::Request(request) => {
                    let request_id = request.id.clone();
                    match handle_request(request) {
                        Ok(response) => {
                            self.connection
                                .sender
                                .send(Message::Response(Response {
                                    id: request_id,
                                    result: Some(response),
                                    error: None,
                                }))
                                .expect("failed to send response to the client");
                        }
                        Err(err) => error!(lsp_event = "failed to handle request", ?err),
                    }
                }
                Message::Response(_) => panic!("Can the client send a response ?"),
                Message::Notification(_) => unimplemented!("Notification not implemented yet"),
            }
        }

        Ok(())
    }
}

fn handle_request(request: Request) -> Result<serde_json::Value, Box<dyn Error>> {
    match request.method.as_str() {
        "textDocument/hover" => Ok(serde_json::json!(&handle_text_document_hover(request)?)),
        unknown => Err(format!("Unknown request method: '{}'", unknown).into()),
    }
}
