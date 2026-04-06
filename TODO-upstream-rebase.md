# upstream v0.22.1 ベースに MCP を載せ直す

## 背景

- upstream が v0.19.0 で **cargo ワークスペース化**（`src/` → `crates/google-workspace-cli/src/`）
- upstream が `mail-builder` クレートに移行し、自前の `MessageBuilder` を廃止
- 265 コミット・108 ファイルのコンフリクトにより通常マージは困難
- **方針: upstream を丸ごとベースにして MCP を載せ直す**

## 作業手順

### 1. upstream/main をベースにブランチ作成

```bash
git fetch upstream
git checkout -b rebase-mcp-on-upstream upstream/main
```

### 2. フォーク独自ファイルを追加

- [ ] `crates/google-workspace-cli/src/mcp_server.rs` — MCP サーバー本体（1,368行）
  - **注意**: `execute_method()` のシグネチャが変更済み
    - `upload_path: Option<&str>` → `upload: Option<UploadSource<'_>>` に変更
    - MCP から呼ぶ場合は `None` でOK（ファイルアップロードしないため）
  - **注意**: `MessageBuilder` が `mail_builder::MessageBuilder` に変更済み
    - `handle_gmail_send()` を新 API に書き直す必要あり
    - 参考: `crates/google-workspace-cli/src/helpers/gmail/send.rs` の `create_send_raw_message()`
  - **注意**: `build_raw_send_body()` が廃止 → `dispatch_raw_email()` に統合
    - MCP 用に `dispatch_raw_email` 相当のロジックを組み直す or 関数を `pub(crate)` にして再利用
- [ ] `FORK.md` — 英語版フォーク説明（現行ファイルをそのままコピー）
- [ ] `FORK.ja.md` — 日本語版フォーク説明（現行ファイルをそのままコピー）
- [ ] `CLAUDE.md` — フォーク固有の注記を追記（upstream 版に追記する形）
- [ ] `.github/workflows/sync-upstream.yml` — upstream 同期ワークフロー
- [ ] `.changeset/mcp-helper-tools.md` — changeset

### 3. 既存ファイルの修正

- [ ] `crates/google-workspace-cli/src/main.rs`
  - `mod mcp_server;` 追加
  - `if first_arg == "mcp"` エントリポイント追加（helpers 統合部分の前に配置）
- [ ] `crates/google-workspace-cli/src/helpers/gmail/mod.rs`
  - MCP から使う関数・型を `pub(super)` → `pub(crate)` に変更
  - 対象（upstream の新構造で要確認）:
    - `resolve_mail_method()` （旧 `resolve_send_method()`）
    - `dispatch_raw_email()` （MCP から直接呼ぶなら）
    - `finalize_message()` （MCP で RFC 2822 を組み立てるなら）
    - `Mailbox`, `to_mb_address_list()` 等
    - `ThreadingHeaders`

### 4. mcp_server.rs の `handle_gmail_send()` 書き直し

upstream の `send.rs::create_send_raw_message()` を参考に:

```rust
// 旧（自前 MessageBuilder）
let raw_message = crate::helpers::gmail::MessageBuilder {
    to, subject, from: None, cc, bcc, threading: None, html: false,
}.build(body_text);
let send_body = crate::helpers::gmail::build_raw_send_body(&raw_message, None);

// 新（mail_builder）
let mb = mail_builder::MessageBuilder::new()
    .to(to_mb_address_list(&to_mailboxes))
    .subject(subject);
let mb = apply_optional_headers(mb, None, cc_mailboxes, bcc_mailboxes);
let raw = finalize_message(mb, body_text, false, &[])?;
// → dispatch_raw_email() 相当のロジックで execute_method() 呼び出し
```

### 5. 不要ワークフロー削除

upstream にある以下を削除:
- `.github/workflows/automation.yml`
- `.github/workflows/cla.yml`
- `.github/workflows/coverage.yml`
- `.github/workflows/generate-skills.yml`
- `.github/workflows/publish-skills.yml`
- `.github/workflows/release-changesets.yml`
- `.github/workflows/release.yml`
- `.github/workflows/stale.yml`

### 6. 検証

```bash
cargo clippy -- -D warnings
cargo test
```

### 7. main にマージ

```bash
git checkout main
git reset --hard rebase-mcp-on-upstream
# または merge
git push origin main --force-with-lease
```

## 参照ファイル（現行フォーク）

作業中に参照すべき現行ファイル（`main` ブランチ）:
- `src/mcp_server.rs` — MCP 実装の元ネタ
- `src/main.rs:141-143` — MCP エントリポイント
- `FORK.md`, `FORK.ja.md` — そのままコピー
- `CLAUDE.md` — フォーク固有部分を抽出して追記
- `.github/workflows/sync-upstream.yml` — そのままコピー

## upstream の新 API ポイント

| 旧（v0.16.0） | 新（v0.22.1） |
|---|---|
| `src/` | `crates/google-workspace-cli/src/` |
| 自前 `MessageBuilder` struct | `mail_builder::MessageBuilder` |
| `build_raw_send_body()` | `dispatch_raw_email()` に統合 |
| `resolve_send_method()` | `resolve_mail_method(doc, draft)` |
| `upload_path: Option<&str>` + `upload_content_type: Option<&str>` | `upload: Option<UploadSource<'_>>` |
| `Cargo.toml`（単一 crate） | `Cargo.toml`（workspace） + `crates/*/Cargo.toml` |
