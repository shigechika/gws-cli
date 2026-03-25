# Fork: MCP サーバー機能を温存した gws

このリポジトリは [googleworkspace/cli](https://github.com/googleworkspace/cli) のフォークです。

[English version](FORK.md)

upstream が削除した **MCP（Model Context Protocol）サーバー機能** を独自にメンテナンスし、AI エージェントから Google Workspace API を直接呼び出せる状態を維持しています。

## upstream との差分

| 項目 | upstream | このフォーク |
|---|---|---|
| MCP サーバー (`gws mcp`) | 削除済み | 維持・メンテナンス中 |
| MCP helper tools (`--helpers`) | なし | `gmail_send` 等を独自実装 |
| CI/CD ワークフロー | upstream 環境依存 | 最小構成（CI + Policy + Sync） |

### MCP サーバー

Discovery Document から動的にツールを生成し、stdio 経由で MCP プロトコルを提供します。

```bash
# Gmail の MCP サーバーを起動（helper tool 付き）
gws mcp -s gmail --helpers

# 複数サービスを同時に提供
gws mcp -s gmail -s drive -s calendar --helpers

# compact モード（サービスごとに1ツール）
gws mcp -s gmail --tool-mode compact
```

### MCP helper tools

`--helpers` フラグで有効化される便利ツールです。Discovery API の raw tool に加え、RFC 2822 構築や base64 エンコード等の面倒な処理を自動化します。

| ツール名 | 説明 |
|---|---|
| `gmail_send` | メール送信。to/subject/body を渡すだけで RFC 2822 フォーマット・base64url エンコードを自動処理 |

## インストール

upstream の npm パッケージには MCP 機能が含まれていないため、ソースからビルドしてください。

```bash
# GitHub から直接インストール（推奨）
cargo install --git https://github.com/shigechika/gws-cli --locked
```

ローカルに clone 済みの場合は、ワーキングツリーからインストール:

```bash
cd gws-cli
cargo install --path .
```

`~/.cargo/bin/gws` にバイナリがインストールされます。`cargo build --release` は `target/release/gws` にビルドするだけで `~/.cargo/bin/` は**更新されない**点に注意してください。

## Claude での使い方

**Claude Code** — `~/.claude.json` に追加:

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

**Claude Desktop** — `~/Library/Application Support/Claude/claude_desktop_config.json`（macOS）に追加:

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

## upstream MCP 定点観測

| 時期 | 出来事 |
|---|---|
| 2026-03 | `fix!: Remove MCP server mode` — upstream が breaking change として MCP サーバーを削除 |
| 2026-03 | ブランチ `fix/mcp-hyphen-tool-names` が upstream に出現 — ツール名ハイフン化で MCP 復活の兆し |
| 2026-03 | 同ブランチがマージされずに削除 — upstream での MCP 復活は見送り |

## upstream 同期方針

- 毎週月曜に GitHub Actions で upstream/main を自動マージ
- コンフリクト発生時は PR を作成して手動解決
- MCP 関連コード（`src/mcp_server.rs`、`pub(crate)` 可視性）の温存を最優先
- upstream のコミットメッセージから `#番号` 参照を除去（クロスリファレンス防止）
