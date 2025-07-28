use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tool {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Handshake {
    pub version: String,
    pub tools: Vec<Tool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ToolCall {
    pub tool_name: String,
    // Using serde_json::Value to allow for flexible arguments
    pub parameters: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ToolResponse {
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_handshake_serialization() {
        let handshake = Handshake {
            version: "1.0".to_string(),
            tools: vec![Tool {
                name: "ping".to_string(),
                description: Some("A simple test tool".to_string()),
            }],
        };
        let json = serde_json::to_string(&handshake).unwrap();
        assert!(json.contains("\"version\":\"1.0\""));
        assert!(json.contains("\"toolName\":\"ping\""));
        assert!(json.contains("\"description\":\"A simple test tool\""));
    }

    #[test]
    fn test_tool_call_deserialization() {
        let json_str = r#"{
            "toolName": "date",
            "parameters": {
                "format": "YYYY-MM-DD"
            }
        }"#;
        let tool_call: ToolCall = serde_json::from_str(json_str).unwrap();
        assert_eq!(tool_call.tool_name, "date");
        assert_eq!(tool_call.parameters, json!({ "format": "YYYY-MM-DD" }));
    }

    #[test]
    fn test_tool_response_serialization() {
        let response = ToolResponse {
            content: "pong".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(json, r#"{"content":"pong"}"#);
    }
}
