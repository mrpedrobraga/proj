use crate::{project::ProjectKind, server::ProjectServer};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    DidOpenTextDocumentParams, Hover, HoverParams, MarkupContent, TextDocumentSyncKind,
};
use tower_lsp::{
    Client, LanguageServer,
    lsp_types::{
        InitializeParams, InitializeResult, InitializedParams, MessageType, ServerCapabilities,
    },
};

pub use tower_lsp;

#[derive(Debug)]
pub struct ProjectRepl<P: ProjectKind> {
    pub server: ProjectServer<P>,
    pub client: Client,
}

#[tower_lsp::async_trait]
impl<P: ProjectKind + std::fmt::Debug + 'static> LanguageServer for ProjectRepl<P> {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                // TODO: Make incremental :-)
                text_document_sync: Some(tower_lsp::lsp_types::TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                // position_encoding: todo!(),
                // text_document_sync: todo!(),
                // selection_range_provider: todo!(),
                hover_provider: Some(tower_lsp::lsp_types::HoverProviderCapability::Simple(true)),
                // completion_provider: todo!(),
                // signature_help_provider: todo!(),
                // definition_provider: todo!(),
                // type_definition_provider: todo!(),
                // implementation_provider: todo!(),
                // references_provider: todo!(),
                // document_highlight_provider: todo!(),
                // document_symbol_provider: todo!(),
                // workspace_symbol_provider: todo!(),
                // code_action_provider: todo!(),
                // code_lens_provider: todo!(),
                // document_formatting_provider: todo!(),
                // document_range_formatting_provider: todo!(),
                // document_on_type_formatting_provider: todo!(),
                // rename_provider: todo!(),
                // document_link_provider: todo!(),
                // color_provider: todo!(),
                // folding_range_provider: todo!(),
                // declaration_provider: todo!(),
                // execute_command_provider: todo!(),
                // workspace: todo!(),
                // call_hierarchy_provider: todo!(),
                // semantic_tokens_provider: todo!(),
                // moniker_provider: todo!(),
                // linked_editing_range_provider: todo!(),
                // inline_value_provider: todo!(),
                // inlay_hint_provider: todo!(),
                // diagnostic_provider: todo!(),
                // experimental: todo!(),
                ..Default::default()
            },
            server_info: None,
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Language Server started!")
            .await;
    }

    async fn did_open(&self, _param: DidOpenTextDocumentParams) {
        /* Maybe cache the content of the relevant file? */
    }

    async fn hover(&self, param: HoverParams) -> Result<Option<Hover>> {
        let text_document_uri = param.text_document_position_params.text_document.uri;
        assert_eq!(text_document_uri.scheme(), "file");
        let file_path = text_document_uri.to_file_path().expect("Not a file path?");
        if let Some(view) = &self.server.view {
            let module = view.modules.module_at_file_path(file_path.clone());
            let module_path = &module.unwrap().internal_path;

            return Ok(Some(Hover {
                contents: tower_lsp::lsp_types::HoverContents::Markup(MarkupContent {
                    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                    value: format!("You are hovering over `{module_path:?}`."),
                }),
                range: None,
            }));
        }

        Ok(None)
    }

    async fn shutdown(&self) -> Result<()> {
        self.client
            .log_message(MessageType::INFO, "Language Server is shutting down.")
            .await;
        Ok(())
    }
}
