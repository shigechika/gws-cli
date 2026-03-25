When contributing to this repository, you must strictly follow all guidelines outlined in the AGENTS.md file.

## Fork-specific Notes

This is a fork of `googleworkspace/cli` that maintains MCP server support. See `FORK.md` for details.

### Upstream Merge Checklist

After merging upstream/main, fix MCP compilation errors:
1. `crates/google-workspace-cli/src/mcp_server.rs` — match new arguments in `executor::execute_method()` calls
2. `crates/google-workspace-cli/src/mcp_server.rs` — match new fields in Gmail helper structs
3. `crates/google-workspace-cli/src/helpers/gmail/mod.rs` — ensure `pub(crate)` visibility is not reverted to `pub(super)`
4. `pub(crate)` targets: `Mailbox`, `to_mb_address_list`, `apply_optional_headers`, `finalize_message`, `resolve_mail_method`, `Attachment`
5. Run `cargo clippy -- -D warnings && cargo test` to verify
6. If conflicts exceed ~20 files, consider rebasing: checkout upstream/main as new branch, re-apply MCP changes, reset main

### Project Structure

- Cargo workspace: `crates/google-workspace-cli/` (binary) + `crates/google-workspace/` (library)
- MCP server: `crates/google-workspace-cli/src/mcp_server.rs`
- Local install: `cargo install --path crates/google-workspace-cli`

### GitHub Actions

- Only 3 workflows exist: `ci.yml`, `policy.yml`, `sync-upstream.yml`
- `gh workflow list` may show upstream workflows — use `gh api repos/<owner>/<repo>/actions/workflows` to check actual fork workflows
- `gh run list` may show upstream runs — filter with `--branch=main` for fork-specific results
