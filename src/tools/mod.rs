mod check;
mod parse;

use check::CheckTool;
use parse::ParseTool;

rust_mcp_sdk::tool_box!(MyTools, [ParseTool, CheckTool]);
