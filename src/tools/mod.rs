mod check;
mod docs;
mod parse;

use check::CheckTool;
use docs::DocsTool;
use parse::ParseTool;

rust_mcp_sdk::tool_box!(MyTools, [DocsTool, ParseTool, CheckTool]);

// ---

#[derive(Debug)]
pub struct StringError(pub String);

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for StringError {}
