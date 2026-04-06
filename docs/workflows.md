# GitHub Workflows Documentation

このドキュメントは `.github/workflows` ディレクトリ内のワークフロー設定について説明します。

## 概要

このリポジトリには、GitHub Actionsを使用した自動化ワークフローが設定されています。

## ワークフローファイル

### claude.yml

**目的**: Claude Code Actionを使用したAIアシスタント機能を提供

**トリガー条件**:
- Issue コメントが作成された時（`@claude` を含む）
- Pull Request レビューコメントが作成された時（`@claude` を含む）
- Issue が開かれた時またはアサインされた時（`@claude` を含む）
- Pull Request レビューが提出された時（`@claude` を含む）

**実行環境**:
- OS: Ubuntu Latest
- Node.js: 24（強制設定）

**権限**:
- `contents: write` - リポジトリの内容を読み書き
- `pull-requests: write` - プルリクエストの作成・編集
- `issues: write` - Issue の作成・編集
- `id-token: write` - IDトークンの書き込み

**実行ステップ**:
1. **Checkout**: リポジトリのコードをチェックアウト（最新1コミットのみ）
2. **Run Claude**: Claude Code Actionを実行
   - モデル: `claude-sonnet-4-20250514`
   - APIキー: シークレットから取得 (`ANTHROPIC_API_KEY`)

## 設定方法

### 必要なシークレット

このワークフローを使用するには、以下のシークレットをリポジトリ設定で設定する必要があります：

- `ANTHROPIC_API_KEY`: Anthropic API のアクセスキー

### 使用方法

1. Issue またはプルリクエストのコメントで `@claude` とメンション
2. Claudeが自動的に応答し、コードの分析や修正を実行
3. 必要に応じて新しいブランチを作成し、プルリクエストを提出

## 技術詳細

### ワークフロー条件

ワークフローは以下の条件で実行されます：

```yaml
if: |
  (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
  (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
  (github.event_name == 'pull_request_review' && contains(github.event.review.body, '@claude')) ||
  (github.event_name == 'issues' && contains(github.event.issue.body, '@claude'))
```

### 使用されるアクション

- `actions/checkout@v4`: リポジトリのチェックアウト
- `anthropics/claude-code-action@beta`: Claude Code Actionの実行

## 注意事項

- このワークフローはベータ版のアクション（`@beta`）を使用しています
- Node.js 24の使用が強制されており、非推奨警告を抑制しています
- 実行には Anthropic API キーが必要です