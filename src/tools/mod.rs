mod parse;

use parse::{ParseTool, CheckTool};

rust_mcp_sdk::tool_box!(MyTools, [ParseTool, CheckTool]);
