use lsp_bridge::{LspBridge, LspServerConfig};
use serde_json::json;
use std::{env, path::PathBuf, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_server().await;

    let target_dir = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
            manifest_dir
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("target")
        });
    let binary_path = target_dir.join("debug").join("md-server");
    let server_cwd = "/home/mrpedrobraga/Development/proj/crates/md-server";

    let server_config = LspServerConfig::new()
        .command(binary_path.to_string_lossy().into_owned())
        .root_path(server_cwd)
        .workspace_folder(server_cwd)
        .env("RUST_BACKTRACE", "1")
        .env("RUST_LOG", "trace")
        .initialization_options(json!({
            "capabilities": {
                "workspace": {},
                "textDocument": {
                    "hover": { "contentFormat": ["markdown", "plaintext"] }
                }
            }
        }));

    let mut bridge = LspBridge::new();

    println!("Registering server...");
    let server_id = bridge.register_server("markdown", server_config).await?;

    println!("Starting server process...");
    bridge.start_server(&server_id).await?;

    sleep(Duration::from_millis(200)).await;

    let file_uri =
        "file:///home/mrpedrobraga/Development/proj/crates/md-server/projects/example-project/README.md";
    let file_content = "# Hello Markdown";

    println!("Opening document...");
    bridge
        .open_document(&server_id, file_uri, file_content)
        .await?;

    sleep(Duration::from_millis(50)).await;

    println!("Sending hover request...");

    let hover = bridge
        .get_hover(&server_id, file_uri, lsp_types::Position::new(0, 0))
        .await;
    let _ = dbg!(hover);

    Ok(())
}

async fn build_server() {
    let cargo_exe = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let build_status = tokio::process::Command::new(&cargo_exe)
        .args(["build", "-p", "md-server", "--bin", "md-server", "-q"])
        .status()
        .await
        .expect("Failed to build server binary");
    assert!(build_status.success(), "Server compilation failed!");
}
