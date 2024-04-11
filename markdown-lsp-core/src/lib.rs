use lsp_server::{Connection, IoThreads, Message, Request, Response, ResponseError};
use lsp_types::{InitializeResult, ServerInfo};
use std::error::Error;
use text_document_hover::handle_text_document_hover;
use tracing::info;

mod server_configuration;
mod text_document_hover;

pub struct LspContext {
    connection: Connection,
    io_threads: IoThreads,
}

impl Default for LspContext {
    fn default() -> Self {
        let (connection, io_threads) = Connection::stdio();
        Self {
            connection,
            io_threads,
        }
    }
}

impl LspContext {
    pub fn init(&self) -> Result<(), Box<dyn Error>> {
        // wait for client initialization request
        let (req_id, _client_init_params) = self.connection.initialize_start()?;
        info!(
            lsp_event = "received client initialization request",
            ?req_id
        );

        // send initialization response to the client
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
                            self.connection.sender.send(Message::Response(Response {
                                id: request_id,
                                result: Some(response),
                                error: None,
                            }))?
                        }
                        Err(err) => self.connection.sender.send(Message::Response(Response {
                            id: request_id,
                            result: None,
                            error: Some(ResponseError {
                                // internal error
                                // TODO: return real error
                                code: -32603,
                                message: err.to_string(),
                                data: None,
                            }),
                        }))?,
                    }
                }
                Message::Response(_) => panic!("Can the client send a response ?"),
                Message::Notification(_) => { /* TODO */ }
            }
        }

        Ok(())
    }

    pub fn shutdown_gracefully(self) -> Result<(), Box<dyn Error>> {
        self.io_threads.join()?;
        Ok(())
    }
}

fn handle_request(request: Request) -> Result<serde_json::Value, Box<dyn Error>> {
    match request.method.as_str() {
        "textDocument/hover" => Ok(serde_json::json!(&handle_text_document_hover(request)?)),
        unknown => Err(format!("Unknown request method: '{}'", unknown).into()),
    }
}
