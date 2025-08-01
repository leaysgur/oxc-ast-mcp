use oxc_allocator::Allocator;
use oxc_parser::{ParseOptions, Parser};
use oxc_span::SourceType;
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

use super::{MyTool, StringError};

#[mcp_tool(
    name = "parse",
    title = "A tool that parses JS(X) or TS(X) code into OXC AST.",
    description = "Accepts a code snippet and extension. Extension should be one of `js`, `mjs`, `cjs`, `jsx`, `ts`, `mts`, `cts`, `tsx`. You can use `docs` tool to see the documentation for each AST node.",
    meta = r#"{"version": "1.0"}"#
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct ParseTool {
    code: String,
    ext: String,
}

impl MyTool for ParseTool {
    fn call(&self) -> Result<CallToolResult, CallToolError> {
        let Self { code, ext } = self;

        let source_type = SourceType::from_extension(ext).map_err(CallToolError::new)?;

        let allocator = Allocator::default();
        let parser = Parser::new(&allocator, code, source_type).with_options(ParseOptions {
            parse_regular_expression: true,
            ..Default::default()
        });
        let ret = parser.parse();

        if !ret.errors.is_empty() {
            return Err(CallToolError::new(StringError(
                "Parse failed with errors. You can see diagnostics with `check` tool.".to_string(),
            )));
        }

        let ast_string = format!("{:#?}", ret.program);
        Ok(CallToolResult::text_content(vec![ast_string.into()]))
    }
}
