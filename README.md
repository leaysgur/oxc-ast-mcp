# oxc-ast-mcp

Unofficial+experimental MCP server for OXC parser.

## Available tools

- `parse`: Parse the code snippet and returns AST
- `check`: Check the code and returns syntactic and semantic diagnostics

## Expected usecases

- As a companion when implementing tools such as `linter` or `formatter` in the OXC repository
  - By understanding the AST structure, agent can handle appropriate AST nodes
- For operation verification after having an agent implement code
  - Discover basic syntacitic and semantic issues before lint, execute
- and more...
