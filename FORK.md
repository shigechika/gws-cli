# Fork: gws with MCP server support

This repository is a fork of [googleworkspace/cli](https://github.com/googleworkspace/cli).

It maintains the **MCP (Model Context Protocol) server** that upstream removed, allowing AI agents to call Google Workspace APIs directly.

[日本語版はこちら](FORK.ja.md)

## Differences from upstream

| Feature | upstream | This fork |
|---|---|---|
| MCP server (`gws mcp`) | Removed | Maintained |
| MCP helper tools (`--helpers`) | N/A | `gmail_send` and more |
| CI/CD workflows | Upstream-specific | Minimal (CI + Policy + Sync) |

### MCP server

Dynamically generates tools from Discovery Documents and serves them via the MCP protocol over stdio.

```bash
# Start MCP server for Gmail with helper tools
gws mcp -s gmail --helpers

# Serve multiple services
gws mcp -s gmail -s drive -s calendar --helpers

# Compact mode (one tool per service)
gws mcp -s gmail --tool-mode compact
```

### MCP helper tools

Enabled with the `--helpers` flag. These provide high-level operations on top of the raw Discovery API tools, automating tedious tasks like RFC 2822 formatting and base64url encoding.

| Tool | Description |
|---|---|
| `gmail_send` | Send email. Just pass to/subject/body — RFC 2822 formatting and base64url encoding are handled automatically |

## Installation

The upstream npm package does not include MCP support. Build from source:

```bash
# Install directly from GitHub (recommended)
cargo install --git https://github.com/shigechika/gws-cli --locked
```

If you cloned the repository locally, install from the working tree:

```bash
cd gws-cli
cargo install --path crates/google-workspace-cli
```

This installs the binary to `~/.cargo/bin/gws`. Note that `cargo build --release` only builds to `target/release/gws` and does **not** update `~/.cargo/bin/`.

## Usage with Claude

**Claude Code** — add to `~/.claude.json`:

```json
{
  "mcpServers": {
    "gws": {
      "command": "gws",
      "args": ["mcp", "-s", "gmail", "-s", "drive", "-s", "calendar", "--helpers"]
    }
  }
}
```

**Claude Desktop** — add to `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS):

```json
{
  "mcpServers": {
    "gws": {
      "command": "gws",
      "args": ["mcp", "-s", "gmail", "-s", "drive", "-s", "calendar", "--helpers"]
    }
  }
}
```

## Upstream MCP timeline

| Date | Event |
|---|---|
| 2026-03-04 | `feat: add gws mcp server` — MCP server added to upstream |
| 2026-03-05 | Branch `fix/mcp-hyphen-tool-names` appeared in upstream — tool name separator change from underscore to hyphen |
| 2026-03-06 | `fix!: Remove MCP server mode` — MCP server removed from upstream as a breaking change, just 2 days after introduction |
| 2026-03-06 | Branch `fix/mcp-hyphen-tool-names` deleted without being merged — MCP remains absent from upstream |

## Upstream sync policy

- Weekly auto-merge from upstream/main via GitHub Actions (every Monday)
- Conflicts trigger a PR for manual resolution
- MCP-related code (`src/mcp_server.rs`, `pub(crate)` visibility) is preserved as top priority
- Issue/PR number references (`#123`) are stripped from upstream commit messages to prevent cross-references
