use std::str::FromStr;

use crate::project::ModuleEntry;
use crate::server::ProjectServer;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    CodeAction, CodeActionKind, CodeActionOptions, CodeActionOrCommand, CodeActionParams,
    CodeDescription, Diagnostic, DiagnosticOptions, DiagnosticSeverity, DidOpenTextDocumentParams,
    DocumentDiagnosticParams, DocumentDiagnosticReportResult, FullDocumentDiagnosticReport, Hover,
    HoverParams, MarkupContent, OneOf, OptionalVersionedTextDocumentIdentifier, Position, Range,
    RelatedFullDocumentDiagnosticReport, TextDocumentEdit, TextDocumentSyncKind, TextEdit, Url,
    WorkDoneProgressOptions, WorkspaceEdit,
};
use tower_lsp::{
    lsp_types::{
        InitializeParams, InitializeResult, InitializedParams, MessageType, ServerCapabilities,
    },
    Client, LanguageServer,
};

pub use tower_lsp;

pub struct ProjectRepl {
    pub server: ProjectServer,
    pub client: Client,
}

impl ProjectRepl {
    /// Returns a module from the open projects given its URI if such a module exists.
    pub fn module_from_uri(&self, uri: &Url) -> Option<&ModuleEntry> {
        let text_document_uri = uri;
        assert_eq!(text_document_uri.scheme(), "file");
        let file_path = text_document_uri.to_file_path().expect("Not a file path?");

        self.server
            .open_projects
            .iter()
            .filter_map(|p| p.module_at_file_path(&file_path))
            .next()
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for ProjectRepl {
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
                code_action_provider: Some(
                    tower_lsp::lsp_types::CodeActionProviderCapability::Options(
                        CodeActionOptions {
                            code_action_kinds: Some(vec![CodeActionKind::REFACTOR_REWRITE]),
                            work_done_progress_options: WorkDoneProgressOptions {
                                work_done_progress: None,
                            },
                            resolve_provider: Some(true),
                        },
                    ),
                ),
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
                diagnostic_provider: Some(
                    tower_lsp::lsp_types::DiagnosticServerCapabilities::Options(
                        DiagnosticOptions {
                            identifier: None, // TODO: Add an identifier here :-)
                            inter_file_dependencies: true,
                            workspace_diagnostics: false,
                            work_done_progress_options: WorkDoneProgressOptions::default(),
                        },
                    ),
                ),
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
        if let Some(module) =
            self.module_from_uri(&param.text_document_position_params.text_document.uri)
        {
            let hover_info = module
                .content
                .hover_information_at(param.text_document_position_params.position.into());

            return Ok(hover_info.map(|info| Hover {
                contents: tower_lsp::lsp_types::HoverContents::Markup(MarkupContent {
                    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                    value: info.text,
                }),
                range: info.range.map(|range| tower_lsp::lsp_types::Range {
                    start: range.0.into(),
                    end: range.1.into(),
                }),
            }));
        }

        Ok(None)
    }

    async fn diagnostic(
        &self,
        _params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult> {
        let items = vec![Diagnostic {
            range: Range {
                start: Position::new(0, 99),
                end: Position::new(0, 99),
            },
            severity: Some(DiagnosticSeverity::INFORMATION),
            code: Some(tower_lsp::lsp_types::NumberOrString::Number(42)),
            code_description: Some(CodeDescription {
                href: Url::from_str("https://mrpedrobraga.com/error").unwrap(),
            }),
            source: Some(String::from("proj")),
            message: "This line could be uppercase. Like, if you wanted I guess. You could do it."
                .to_string(),
            related_information: None,
            tags: None,
            data: None,
        }];

        Ok(DocumentDiagnosticReportResult::Report(
            tower_lsp::lsp_types::DocumentDiagnosticReport::Full(
                RelatedFullDocumentDiagnosticReport {
                    related_documents: None,
                    full_document_diagnostic_report: FullDocumentDiagnosticReport {
                        result_id: None,
                        items,
                    },
                },
            ),
        ))
    }

    async fn code_action(
        &self,
        params: CodeActionParams,
    ) -> Result<Option<Vec<CodeActionOrCommand>>> {
        if let Some(_module) = self.module_from_uri(&params.text_document.uri) {
            let new_line_text = _module.content.nth_line(0).unwrap().to_uppercase();

            let c_a = CodeActionOrCommand::CodeAction(CodeAction {
                title: "Make line uppercase.".to_string(),
                kind: Some(CodeActionKind::REFACTOR_REWRITE),
                diagnostics: Some(vec![Diagnostic {
                    range: Range {
                        start: Position::new(0, 99),
                        end: Position::new(0, 99),
                    },
                    severity: Some(DiagnosticSeverity::INFORMATION),
                    code: Some(tower_lsp::lsp_types::NumberOrString::Number(42)),
                    code_description: Some(CodeDescription {
                        href: Url::from_str("https://mrpedrobraga.com/error").unwrap(),
                    }),
                    source: Some(String::from("proj")),
                    message:
                        "This line could be uppercase. Like, if you wanted I guess. You could do it."
                            .to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                }]),
                edit: Some(WorkspaceEdit {
                    changes: None,
                    document_changes: Some(tower_lsp::lsp_types::DocumentChanges::Edits(vec![ TextDocumentEdit { text_document: OptionalVersionedTextDocumentIdentifier { uri: params.text_document.uri, version: None }, edits: vec![ OneOf::Left(TextEdit{ range: Range {
                start: Position::new(0, 0),
                end: Position::new(0, 99),
            }, new_text: new_line_text })] } ])),
                    change_annotations: None,
                }),
                command: None,
                is_preferred: Some(true),
                disabled: None,
                data: None,
            });

            return Ok(Some(vec![c_a]));
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
