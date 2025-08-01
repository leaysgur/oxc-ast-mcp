use async_trait::async_trait;
use rust_mcp_sdk::{
    McpServer, StdioTransport, TransportOptions,
    error::SdkResult,
    mcp_server::ServerHandler,
    mcp_server::server_runtime,
    schema::{
        CallToolRequest, CallToolResult, Implementation, InitializeResult, LATEST_PROTOCOL_VERSION,
        ListToolsRequest, ListToolsResult, RpcError, ServerCapabilities, ServerCapabilitiesTools,
        schema_utils::CallToolError,
    },
};

use oxc_ast_mcp::tools::{MyTools, MyTool};

#[tokio::main]
async fn main() -> SdkResult<()> {
    let server_details = InitializeResult {
        server_info: Implementation {
            name: "OXC AST MCP Server".to_string(),
            version: "0.1.0".to_string(),
            title: None,
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        meta: None,
        instructions: Some("This server helps you to understand how ASTs are structured by OXC parser. Also useful to quick check for your code which should be valid syntax.".to_string()),
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    };

    let server = server_runtime::create_server(
        server_details,
        StdioTransport::new(TransportOptions::default())?,
        MyServerHandler,
    );
    server.start().await
}

// ---

struct MyServerHandler;

#[async_trait]
impl ServerHandler for MyServerHandler {
    async fn handle_list_tools_request(
        &self,
        _request: ListToolsRequest,
        _runtime: &dyn McpServer,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools: MyTools::tools(),
        })
    }

    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _runtime: &dyn McpServer,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let tool_params = MyTools::try_from(request.params).map_err(CallToolError::new)?;

        match tool_params {
            MyTools::DocsTool(docs_tool) => docs_tool.call(),
            MyTools::ParseTool(parse_tool) => parse_tool.call(),
            MyTools::CheckTool(check_tool) => check_tool.call(),
        }
    }
}
