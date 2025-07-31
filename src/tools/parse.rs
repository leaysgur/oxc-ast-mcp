use std::sync::Arc;

use oxc_allocator::Allocator;
use oxc_parser::{ParseOptions, Parser};
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolResult, schema_utils::CallToolError},
};

#[mcp_tool(
    name = "parse",
    title = "A tool that parses JS(X) or TS(X) code into OXC AST.",
    description = "Accepts a code snippet and extension. Extension should be one of `js`, `mjs`, `cjs`, `jsx`, `ts`, `mts`, `cts`, `tsx`.",
    meta = r#"{"version": "1.0"}"#
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct ParseTool {
    code: String,
    ext: String,
}

impl ParseTool {
    pub fn call(&self) -> Result<CallToolResult, CallToolError> {
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

#[mcp_tool(
    name = "check",
    title = "A tool that checks JS(X) or TS(X) code has syntactic and/or semantic errors or NOT.",
    description = "Accepts a code snippet and extension. Extension should be one of `js`, `mjs`, `cjs`, `jsx`, `ts`, `mts`, `cts`, `tsx`. Returns both syntactic and semantic errors by default, but can also disable semantic checks if `check_semantic` is set to `false.",
    meta = r#"{"version": "1.0"}"#
)]
#[derive(Debug, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct CheckTool {
    code: String,
    ext: String,
    check_semantic: Option<bool>,
}

impl CheckTool {
    pub fn call(&self) -> Result<CallToolResult, CallToolError> {
        let Self {
            code,
            ext,
            check_semantic,
        } = self;

        let source_text = Arc::new(code.clone());
        let source_type = SourceType::from_extension(ext).map_err(CallToolError::new)?;

        let allocator = Allocator::default();
        let parser =
            Parser::new(&allocator, &source_text, source_type).with_options(ParseOptions {
                parse_regular_expression: true,
                ..Default::default()
            });
        let ret = parser.parse();

        let mut diagnostics = vec![];
        diagnostics.push("# Syntactic errors".to_string());
        if ret.errors.is_empty() {
            diagnostics.push("No syntactic errors found.".to_string());
        } else {
            for error in ret.errors {
                let diagnostic = error.with_source_code(source_text.clone());
                diagnostics.push(format!("{diagnostic:?}"));
            }
        }
        diagnostics.push("".to_string());

        if check_semantic.unwrap_or(true) {
            let semantic = SemanticBuilder::new()
                .with_check_syntax_error(true)
                .build(&ret.program);

            diagnostics.push("# Semantic errors".to_string());
            if semantic.errors.is_empty() {
                diagnostics.push("No semantic errors found.".to_string());
            } else {
                for error in semantic.errors {
                    let diagnostic = error.with_source_code(source_text.clone());
                    diagnostics.push(format!("{diagnostic:?}"));
                }
            }
            diagnostics.push("".to_string());
        }

        Ok(CallToolResult::text_content(vec![
            diagnostics.join("\n").into(),
        ]))
    }
}

// ---

#[derive(Debug)]
struct StringError(String);

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for StringError {}
