use std::io::{self, BufRead, Write};
pub mod mcp;
use mcp::{Handshake, Tool};

fn main() {
    // Send the MCP handshake to tell Cursor which tools are available.
    let handshake = Handshake {
        version: "1.0".to_string(),
        tools: vec![
            Tool {
                name: "ping".to_string(),
                description: Some("Responds with 'pong'.".to_string()),
            },
            Tool {
                name: "date".to_string(),
                description: Some("Responds with the current date.".to_string()),
            },
        ],
    };

    if let Ok(json) = serde_json::to_string(&handshake) {
        println!("{json}");
        io::stdout().flush().expect("Failed to flush stdout");
    }

    // This loop will be replaced in the next step to handle MCP messages.
    // For now, it just keeps the process alive.
    let stdin = io::stdin();
    for _line in stdin.lock().lines() {
        // In the next step, we will parse this line as an MCP message.
    }
}
