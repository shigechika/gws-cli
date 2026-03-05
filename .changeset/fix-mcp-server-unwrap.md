---
"@googleworkspace/cli": patch
---

fix: replace unwrap() calls with proper error handling in MCP server

Replaced four `unwrap()` calls in `mcp_server.rs` that could panic the MCP
server process with graceful error handling. Also added a warning log when
authentication silently falls back to unauthenticated mode.
