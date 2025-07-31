mod check;
mod list;
mod parse;

use check::CheckTool;
use list::ListTool;
use parse::ParseTool;

rust_mcp_sdk::tool_box!(MyTools, [ListTool, ParseTool, CheckTool]);
