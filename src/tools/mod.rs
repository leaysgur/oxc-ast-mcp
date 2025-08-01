mod check;
mod docs;
mod parse;

use rust_mcp_sdk::schema::{CallToolResult, schema_utils::CallToolError};

use check::CheckTool;
use docs::DocsTool;
use parse::ParseTool;

rust_mcp_sdk::tool_box!(MyTools, [DocsTool, ParseTool, CheckTool]);

// ---

pub trait MyTool {
    fn call(&self) -> Result<CallToolResult, CallToolError>;
}

#[derive(Debug)]
pub struct StringError(pub String);

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for StringError {}
