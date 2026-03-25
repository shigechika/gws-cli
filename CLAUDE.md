When contributing to this repository, you must strictly follow all guidelines outlined in the AGENTS.md file.

## Fork-specific Notes

This is a fork of `googleworkspace/cli` that maintains MCP server support. See `FORK.md` for details.

### Upstream Merge Checklist

After merging upstream/main, fix MCP compilation errors:
1. `crates/google-workspace-cli/src/mcp_server.rs` — match new arguments in `executor::execute_method()` calls
2. `crates/google-workspace-cli/src/mcp_server.rs` — match new fields in Gmail helper structs
3. `crates/google-workspace-cli/src/helpers/gmail/mod.rs` — ensure `pub(crate)` visibility is not reverted to `pub(super)`
4. Run `cargo clippy -- -D warnings && cargo test` to verify

### GitHub Actions

- Only 3 workflows exist: `ci.yml`, `policy.yml`, `sync-upstream.yml`
- `gh workflow list` may show upstream workflows — use `gh api repos/<owner>/<repo>/actions/workflows` to check actual fork workflows
- `gh run list` may show upstream runs — filter with `--branch=main` for fork-specific results
