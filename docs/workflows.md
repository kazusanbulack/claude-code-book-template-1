# GitHub Workflows 設定ドキュメント

## 概要

このドキュメントでは、`.github/workflows` ディレクトリ以下のワークフロー設定について説明します。

## Claude Code Action ワークフロー（claude.yml）

### 目的

Claude Code Actionは、GitHub上でClaude AIアシスタントを使用してコード関連のタスクを自動化するためのワークフローです。

### トリガー条件

ワークフローは以下の条件で実行されます：

1. **Issue Comment** - Issue内で `@claude` というメンションが含まれるコメントが作成された時
2. **Pull Request Review Comment** - PRレビューコメントで `@claude` というメンションが含まれるコメントが作成された時
3. **Issues** - Issueが開かれたまたはアサインされた時に、Issue内容に `@claude` が含まれている場合
4. **Pull Request Review** - PRレビューが提出された時に、レビュー内容に `@claude` が含まれている場合

### 実行環境

- **OS**: Ubuntu Latest
- **Node.js**: バージョン24を強制使用（非推奨警告を抑制するため）

### 必要な権限

ワークフローは以下の権限を必要とします：

- `contents: write` - リポジトリのコンテンツを読み取り・書き込み
- `pull-requests: write` - プルリクエストの作成・更新
- `issues: write` - Issueの作成・更新
- `id-token: write` - OpenID Connectトークンの書き込み

### 必要な設定

#### 1. Anthropic API キー

リポジトリのSecretsに以下を設定する必要があります：

```
ANTHROPIC_API_KEY
```

#### 2. 設定手順

1. GitHubリポジトリの「Settings」→「Secrets and variables」→「Actions」に移動
2. 「New repository secret」をクリック
3. Name: `ANTHROPIC_API_KEY`
4. Secret: Anthropic APIキーを入力
5. 「Add secret」をクリック

### 使用方法

1. **Issue での使用**:
   - 新しいIssueを作成し、本文に `@claude` を含める
   - または既存のIssueにコメントで `@claude` を含める

2. **Pull Request での使用**:
   - PRのレビューコメントに `@claude` を含める
   - またはPRレビューで `@claude` を含める

### ワークフローの動作

1. **チェックアウト**: リポジトリのコードをチェックアウト（shallow fetch）
2. **Claude実行**: Claude Code Actionを実行
   - 使用モデル: `claude-sonnet-4-20250514`
   - API認証: 設定されたAnthropic API キーを使用

### 技術的な詳細

#### 環境変数
- `FORCE_JAVASCRIPT_ACTIONS_TO_NODE24: true` - JavaScript ActionsでNode.js 24を強制使用

#### 使用しているAction
- `actions/checkout@v4` - リポジトリのチェックアウト
- `anthropics/claude-code-action@beta` - Claude Code Actionの実行

#### フェッチ設定
- `fetch-depth: 1` - shallow cloneで最新のコミットのみを取得（高速化）

### 注意事項

- ワークフローはbetaバージョンのClaude Code Actionを使用しています
- APIキーが正しく設定されていない場合、ワークフローは失敗します
- `@claude` メンションが含まれていない場合、ワークフローはスキップされます

### トラブルシューティング

#### よくある問題

1. **APIキーエラー**:
   - `ANTHROPIC_API_KEY` が正しく設定されているか確認
   - APIキーが有効期限内であるか確認

2. **権限エラー**:
   - ワークフローファイルの権限設定が正しいか確認
   - リポジトリの設定でActionsの権限が適切に設定されているか確認

3. **トリガーされない**:
   - コメントに `@claude` が正しく含まれているか確認
   - ワークフローのトリガー条件を確認

### セキュリティ考慮事項

- APIキーはSecretsに保存し、コードには直接記述しない
- 最小限の権限のみを付与する
- 定期的にAPIキーをローテーションする

### 更新履歴

- 初版: Claude Code Action ワークフローの基本設定を文書化