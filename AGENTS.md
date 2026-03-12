# AGENTS.md

## Project Overview

`gws` is a Rust CLI tool for interacting with Google Workspace APIs. It dynamically generates its command surface at runtime by parsing Google Discovery Service JSON documents.

> [!IMPORTANT]
> **Dynamic Discovery**: This project does NOT use generated Rust crates (e.g., `google-drive3`) for API interaction. Instead, it fetches the Discovery JSON at runtime and builds `clap` commands dynamically. When adding a new service, you only need to register it in `src/services.rs` and verify the Discovery URL pattern in `src/discovery.rs`. Do NOT add new crates to `Cargo.toml` for standard Google APIs.

> [!NOTE]
> **Package Manager**: Use `pnpm` instead of `npm` for Node.js package management in this repository.

## Build & Test

> [!IMPORTANT]
> **Test Coverage**: The `codecov/patch` check requires that new or modified lines are covered by tests. When adding code, extract testable helper functions rather than embedding logic in `main`/`run` where it's hard to unit-test. Run `cargo test` locally and verify new branches are exercised.

```bash
cargo build          # Build in dev mode
cargo clippy -- -D warnings  # Lint check
cargo test           # Run tests
```

## Commit Messages

> [!IMPORTANT]
> **Issue 番号の除去**: このリポジトリは upstream（`googleworkspace/cli`）のフォークです。コミットメッセージに `#123` のような Issue/PR 番号参照が含まれていると、push 時に GitHub が自動的に upstream の該当 Issue/PR にクロスリファレンスを作成してしまいます。
>
> コミットメッセージを書く際、および upstream のコミットを cherry-pick/merge する際は、必ず `#番号` パターンを除去してください。

```bash
# 悪い例（upstream の Issue #275 にコメントが飛ぶ）
git commit -m 'Revert "fix!: Remove MCP server mode (#275)"'

# 良い例（番号を除去）
git commit -m 'Revert "fix!: Remove MCP server mode"'
```

## Changesets

Every PR must include a changeset file. Create one at `.changeset/<descriptive-name>.md`:

```markdown
---
"@googleworkspace/cli": patch
---

Brief description of the change
```

Use `patch` for fixes/chores, `minor` for new features, `major` for breaking changes. The CI policy check will fail without a changeset.

## Architecture

The CLI uses a **two-phase argument parsing** strategy:

1. Parse argv to extract the service name (e.g., `drive`)
2. Fetch the service's Discovery Document, build a dynamic `clap::Command` tree, then re-parse

### Source Layout

| File                      | Purpose                                                                                   |
| ------------------------- | ----------------------------------------------------------------------------------------- |
| `src/main.rs`             | Entrypoint, two-phase CLI parsing, method resolution                                      |
| `src/discovery.rs`        | Serde models for Discovery Document + fetch/cache                                         |
| `src/services.rs`         | Service alias → Discovery API name/version mapping                                        |
| `src/auth.rs`             | OAuth2 token acquisition via env vars, encrypted credentials, or ADC                      |
| `src/credential_store.rs` | AES-256-GCM encryption/decryption of credential files                                     |
| `src/auth_commands.rs`    | `gws auth` subcommands: `login`, `logout`, `setup`, `status`, `export`                    |
| `src/commands.rs`         | Recursive `clap::Command` builder from Discovery resources                                |
| `src/executor.rs`         | HTTP request construction, response handling, schema validation                           |
| `src/schema.rs`           | `gws schema` command — introspect API method schemas                                      |
| `src/mcp_server.rs`       | MCP stdio server: tool generation from Discovery + helper tools                           |
| `src/error.rs`            | Structured JSON error output                                                              |

## Demo Videos

Demo recordings are generated with [VHS](https://github.com/charmbracelet/vhs) (`.tape` files).

```bash
vhs docs/demo.tape
```

### VHS quoting rules

- Use **double quotes** for simple strings: `Type "gws --help" Enter`
- Use **backtick quotes** when the typed text contains JSON with double quotes:
  ```
  Type `gws drive files list --params '{"pageSize":5}'` Enter
  ```
  `\"` escapes inside double-quoted `Type` strings are **not supported** by VHS and will cause parse errors.

### Scene art

ASCII art title cards live in `art/`. The `scripts/show-art.sh` helper clears the screen and cats the file. Portrait scenes use `scene*.txt`; landscape chapters use `long-*.txt`.

## Input Validation & URL Safety

> [!IMPORTANT]
> This CLI is frequently invoked by AI/LLM agents. Always assume inputs can be adversarial — validate paths against traversal (`../../.ssh`), restrict format strings to allowlists, reject control characters, and encode user values before embedding them in URLs.

> [!NOTE]
> **Environment variables are trusted inputs.** The validation rules above apply to **CLI arguments** that may be passed by untrusted AI agents. Environment variables (e.g. `GOOGLE_WORKSPACE_CLI_CONFIG_DIR`) are set by the user themselves — in their shell profile, `.env` file, or deployment config — and are not subject to path traversal validation. This is consistent with standard conventions like `XDG_CONFIG_HOME`, `CARGO_HOME`, etc.

### Path Safety (`src/validate.rs`)

When adding new helpers or CLI flags that accept file paths, **always validate** using the shared helpers:

| Scenario                               | Validator                                | Rejects                                                              |
| -------------------------------------- | ---------------------------------------- | -------------------------------------------------------------------- |
| File path for writing (`--output-dir`) | `validate::validate_safe_output_dir()`   | Absolute paths, `../` traversal, symlinks outside CWD, control chars |
| File path for reading (`--dir`)        | `validate::validate_safe_dir_path()`     | Absolute paths, `../` traversal, symlinks outside CWD, control chars |
| Enum/allowlist values (`--msg-format`) | clap `value_parser` (see `gmail/mod.rs`) | Any value not in the allowlist                                       |

```rust
// In your argument parser:
if let Some(output_dir) = matches.get_one::<String>("output-dir") {
    crate::validate::validate_safe_output_dir(output_dir)?;
    builder.output_dir(Some(output_dir.clone()));
}
```

### URL Encoding (`src/helpers/mod.rs`)

User-supplied values embedded in URL **path segments** must be percent-encoded. Use the shared helper:

```rust
// CORRECT — encodes slashes, spaces, and special characters
let url = format!(
    "https://www.googleapis.com/drive/v3/files/{}",
    crate::helpers::encode_path_segment(file_id),
);

// WRONG — raw user input in URL path
let url = format!("https://www.googleapis.com/drive/v3/files/{}", file_id);
```

For **query parameters**, use reqwest's `.query()` builder which handles encoding automatically:

```rust
// CORRECT — reqwest encodes query values
client.get(url).query(&[("q", user_query)]).send().await?;

// WRONG — manual string interpolation in query strings
let url = format!("{}?q={}", base_url, user_query);
```

### Resource Name Validation (`src/helpers/mod.rs`)

When a user-supplied string is used as a GCP resource identifier (project ID, topic name, space name, etc.) that gets embedded in a URL path, validate it first:

```rust
// Validates the string does not contain path traversal segments (`..`), control characters, or URL-breaking characters like `?` and `#`.
let project = crate::helpers::validate_resource_name(&project_id)?;
let url = format!("https://pubsub.googleapis.com/v1/projects/{}/topics/my-topic", project);
```

This prevents injection of query parameters, path traversal, or other malicious payloads through resource name arguments like `--project` or `--space`.

### Checklist for New Features

When adding a new helper or CLI command:

1. **File paths** → Use `validate_safe_output_dir` / `validate_safe_dir_path`
2. **Enum flags** → Constrain via clap `value_parser` or `validate_msg_format`
3. **URL path segments** → Use `encode_path_segment()`
4. **Query parameters** → Use reqwest `.query()` builder
5. **Resource names** (project IDs, space names, topic names) → Use `validate_resource_name()`
6. **Write tests** for both the happy path AND the rejection path (e.g., pass `../../.ssh` and assert `Err`)

## MCP Helper Tools

MCP サーバー（`src/mcp_server.rs`）は Discovery Document から自動生成される **raw API tool** に加え、CLI helper の便利機能を再利用した **helper tool** を提供する。

### 設計原則

> [!IMPORTANT]
> **CLI helper コードの再利用**: MCP helper tool は RFC 2822 構築・base64 エンコード等のロジックを独自実装してはならない。`src/helpers/` の既存コードを `pub(crate)` 経由で呼び出すこと。車輪の再発明を防ぎ、セキュリティ対策（ヘッダーインジェクション防止、RFC 2047 エンコード等）の一貫性を保つ。

### アーキテクチャ

```
CLI helper (src/helpers/gmail/)          MCP helper tool (src/mcp_server.rs)
┌─────────────────────────────┐         ┌─────────────────────────────┐
│ +send  --to/--subject/--body│         │ gmail_send  to/subject/body │
│         ↓                   │         │         ↓                   │
│ MessageBuilder::build()     │←─共有──→│ MessageBuilder::build()     │
│ build_raw_send_body()       │←─共有──→│ build_raw_send_body()       │
│ resolve_send_method()       │←─共有──→│ resolve_send_method()       │
│         ↓                   │         │         ↓                   │
│ executor::execute_method()  │         │ executor::execute_method()  │
└─────────────────────────────┘         └─────────────────────────────┘
```

### 可視性ルール

MCP から再利用する型・関数は `pub(crate)` にする。`pub` にはしない（外部クレートに公開する意図がないため）。

| 型/関数 | 定義場所 | 可視性 | 用途 |
|---|---|---|---|
| `MessageBuilder` | `helpers/gmail/mod.rs` | `pub(crate)` | RFC 2822 メッセージ構築 |
| `ThreadingHeaders` | `helpers/gmail/mod.rs` | `pub(crate)` | reply/forward 用スレッディング |
| `build_raw_send_body()` | `helpers/gmail/mod.rs` | `pub(crate)` | base64url エンコード + JSON body 生成 |
| `resolve_send_method()` | `helpers/gmail/mod.rs` | `pub(crate)` | Discovery doc から send メソッド解決 |

### MCP helper tool の追加手順

1. **CLI helper の共有関数を特定** — `src/helpers/<service>/` から再利用可能な関数を見つける
2. **可視性を `pub(crate)` に変更** — `pub(super)` のままだと `mcp_server.rs` から見えない
3. **`append_helper_tools()`** にツール定義を追加 — サービス名で条件分岐
4. **`handle_<service>_<action>()`** を実装 — 引数パース → 共有関数呼び出し → `execute_method()`
5. **`handle_tools_call()`** にルーティング追加 — `--helpers` フラグのゲートを含める
6. **テスト追加** — バリデーション、ゲート、ツール登録の各テスト

### `--helpers` フラグ

Helper tool は `gws mcp --helpers` で有効化される。デフォルトは無効。これにより raw API tool のみで運用したい場合に tool 数が増えない。

```bash
gws mcp -s gmail --helpers          # helper tool 有効
gws mcp -s gmail                    # raw API tool のみ
```

### upstream マージ時の注意

> [!WARNING]
> upstream（`googleworkspace/cli`）が `src/helpers/gmail/mod.rs` の構造体やシグネチャを変更した場合、MCP helper tool が壊れる可能性がある。マージ後は以下を確認すること:
>
> 1. `pub(crate)` に変更した型・関数が `pub(super)` に戻されていないか
> 2. `MessageBuilder` のフィールドが増減していないか（MCP 側のコンストラクタに反映が必要）
> 3. `build_raw_send_body()`, `resolve_send_method()` のシグネチャが変わっていないか
> 4. `cargo test mcp_server` で helper 関連テストが通るか
>
> **対処のコツ**: upstream は `pub(super)` で十分なので変更に気づきにくい。`git diff upstream/main -- src/helpers/gmail/mod.rs` で可視性の巻き戻しをチェックすること。

### upstream ブランチ `fix/mcp-hyphen-tool-names` の追従対応（未マージ）

upstream に MCP 関連の大規模リファクタブランチが存在する。main にマージされた際は以下の対応が必要:

1. **ツール名のハイフン化**: `gmail_send` → `gmail-send`、`gws_discover` → `gws-discover` 等、全ツール名をアンダースコアからハイフン区切りに変更。`handle_tools_call()` のルーティングも合わせて修正。
2. **`walk_resources` の prefix 変更**: `doc.name`（API 名）→ `svc_name`（ユーザー指定エイリアス）に変更済み。
3. **gmail helper の大幅リファクタ**: `reply.rs` と `forward.rs` が削除され `send.rs` に統合。`MessageBuilder`, `OriginalMessage` 等の構造体が変更される。MCP helper の `handle_gmail_send()` がこれらに依存しているため、新しい構造体・シグネチャに合わせて書き直しが必要。
4. **`_helpers` フィールドの巻き戻し**: upstream は `_helpers`（未使用）のままなので、こちらの `helpers` 変更を再適用する。

```bash
# マージ前にブランチの差分を確認
git diff upstream/main -- src/mcp_server.rs src/helpers/gmail/
```

## PR Labels

Use these labels to categorize pull requests and issues:

- `area: discovery` — Discovery document fetching, caching, parsing
- `area: http` — Request execution, URL building, response handling
- `area: docs` — README, contributing guides, documentation
- `area: tui` — Setup wizard, picker, input fields
- `area: distribution` — Nix flake, cargo-dist, npm packaging, install methods
- `area: mcp` — Model Context Protocol server/tools
- `area: auth` — OAuth, credentials, multi-account, ADC
- `area: skills` — AI skill generation and management

## Environment Variables

### Authentication

| Variable | Description |
|---|---|
| `GOOGLE_WORKSPACE_CLI_TOKEN` | Pre-obtained OAuth2 access token (highest priority; bypasses all credential file loading) |
| `GOOGLE_WORKSPACE_CLI_CREDENTIALS_FILE` | Path to OAuth credentials JSON (no default; if unset, falls back to encrypted credentials in `~/.config/gws/`) |
| `GOOGLE_WORKSPACE_CLI_KEYRING_BACKEND` | Keyring backend: `keyring` (default, uses OS keyring with file fallback) or `file` (file only, for Docker/CI/headless) |

| `GOOGLE_APPLICATION_CREDENTIALS` | Standard Google ADC path; used as fallback when no gws-specific credentials are configured |

### Configuration

| Variable | Description |
|---|---|
| `GOOGLE_WORKSPACE_CLI_CONFIG_DIR` | Override the config directory (default: `~/.config/gws`) |

### OAuth Client

| Variable | Description |
|---|---|
| `GOOGLE_WORKSPACE_CLI_CLIENT_ID` | OAuth client ID (for `gws auth login` when no `client_secret.json` is saved) |
| `GOOGLE_WORKSPACE_CLI_CLIENT_SECRET` | OAuth client secret (paired with `CLIENT_ID` above) |

### Sanitization (Model Armor)

| Variable | Description |
|---|---|
| `GOOGLE_WORKSPACE_CLI_SANITIZE_TEMPLATE` | Default Model Armor template (overridden by `--sanitize` flag) |
| `GOOGLE_WORKSPACE_CLI_SANITIZE_MODE` | `warn` (default) or `block` |

### Helpers

| Variable | Description |
|---|---|
| `GOOGLE_WORKSPACE_PROJECT_ID` | GCP project ID override for quota/billing and fallback for helper commands (overridden by `--project` flag) |

All variables can also live in a `.env` file (loaded via `dotenvy`).
