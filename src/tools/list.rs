use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

#[mcp_tool(
    name = "list",
    title = "A tool that lists OXC AST nodes.",
    description = "Accepts a regex string to filter only matched structs and enums.",
    meta = r#"{"version": "1.0"}"#
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct ListTool {
    query: Option<String>,
}

impl ListTool {
    pub fn call(&self) -> Result<CallToolResult, CallToolError> {
        let Self { query } = self;

        // - Read /ast-nodes.generated.json
        //   - { [key]: { docs, body } }
        // - If query is not present return all nodes.
        //   - ```rs
        //   - docs + body (repeat)
        //   - ```
        // - If query is present, filter nodes by key as regex.
        //   - If matches, return the results the same format as above
        // - If query is present, but no matches, filter docs by regex.
        //   - If matches, return the results the same format as above
        // - If reached here, return an not found error.

        let list_string = format!("TODO: Implement: {query:?}");
        Ok(CallToolResult::text_content(vec![list_string.into()]))
    }
}
