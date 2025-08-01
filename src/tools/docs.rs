use regex::Regex;
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};
use serde_json::Value;
use std::collections::HashMap;

use super::{MyTool, StringError};

#[mcp_tool(
    name = "docs",
    title = "A tool that shows OXC AST node documentation.",
    description = "Accepts a regex string to filter only matched structs and enums.",
    meta = r#"{"version": "1.0"}"#
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct DocsTool {
    query: Option<String>,
}

impl MyTool for DocsTool {
    fn call(&self) -> Result<CallToolResult, CallToolError> {
        let Self { query } = self;

        // Read /ast-nodes.generated.json
        let json_content = include_str!("../../ast-nodes.generated.json");

        let nodes: HashMap<String, Value> =
            serde_json::from_str(json_content).map_err(CallToolError::new)?;

        // Helper to extract node data from JSON
        let extract_node_data = |key: &String, node: &Value| -> Option<(String, String, String)> {
            node.as_object().and_then(|node_obj| {
                let docs = node_obj.get("docs").and_then(|v| v.as_str())?;
                let body = node_obj.get("body").and_then(|v| v.as_str())?;
                Some((key.clone(), docs.to_string(), body.to_string()))
            })
        };

        let mut matched_nodes = Vec::new();
        match query {
            // If query is not present, return all nodes
            None => {
                for (key, node) in &nodes {
                    if let Some(node_data) = extract_node_data(key, node) {
                        matched_nodes.push(node_data);
                    }
                }
            }
            // If query is present, filter nodes by key as regex
            Some(query_str) => {
                let regex = Regex::new(query_str).map_err(CallToolError::new)?;

                // First, try to match by key
                for (key, node) in &nodes {
                    if regex.is_match(key) {
                        if let Some(node_data) = extract_node_data(key, node) {
                            matched_nodes.push(node_data);
                        }
                    }
                }

                // If no key matches, try to match by docs
                if matched_nodes.is_empty() {
                    for (key, node) in &nodes {
                        if let Some(node_data) = extract_node_data(key, node) {
                            if regex.is_match(&node_data.1) {
                                matched_nodes.push(node_data);
                            }
                        }
                    }
                }

                // If still no matches, return not found error
                if matched_nodes.is_empty() {
                    return Err(CallToolError::new(StringError(format!(
                        "No nodes found matching query: {query_str}"
                    ))));
                }
            }
        }

        // Sort matched nodes alphabetically by key (A-Z)
        matched_nodes.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        // Format the results as ```rs docs + body (repeat) ```
        let mut result_parts = Vec::new();

        for (_key, docs, body) in matched_nodes {
            result_parts.push("```rs".to_string());
            if !docs.is_empty() {
                result_parts.push(format!("// {}", docs.replace('\n', "\n// ")));
            }
            result_parts.push(body);
            result_parts.push("```".to_string());
            result_parts.push("".to_string()); // Empty line between nodes
        }

        let formatted_result = result_parts.join("\n");
        Ok(CallToolResult::text_content(vec![formatted_result.into()]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docs_tool() {
        // Match all nodes
        let tool1 = DocsTool {
            query: Some(".*".to_string()),
        };
        // No query, should return all nodes
        let tool2 = DocsTool { query: None };

        let to_string = |tool: &DocsTool| {
            tool.call()
                .unwrap()
                .content
                .iter()
                .map(|c| c.as_text_content().unwrap().text.clone())
                .collect::<Vec<_>>()
                .join("\n")
        };

        assert_eq!(to_string(&tool1), to_string(&tool2));
    }

    #[test]
    fn debug() {
        let tool = DocsTool {
            query: Some("JSX.*".to_string()),
        };

        if let Ok(result) = tool.call() {
            for content in &result.content {
                if let Ok(content) = content.as_text_content() {
                    println!("<👻 DEBUG>");
                    println!("{}", content.text);
                    println!("</👻 DEBUG>");
                }
            }
        }
    }
}
