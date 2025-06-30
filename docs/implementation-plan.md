# Rustブログ実装手順書

## 実装ステップ

### Step 1: プロジェクト構造と依存関係の設定

1. **ディレクトリ構造の作成**
   ```
   src/
   ├── main.rs
   ├── lib.rs
   ├── models/
   │   └── article.rs
   ├── storage/
   │   └── file_storage.rs
   └── cli/
       └── commands.rs
   data/
   tests/
   ```

2. **Cargo.tomlに依存関係を追加**
   - `serde`: JSONシリアライゼーション
   - `chrono`: 日時処理
   - `clap`: CLI引数解析
   - `uuid`: 記事ID生成

### Step 2: 記事モデルの実装

1. **Article構造体の定義** (`src/models/article.rs`)
   - id: String (UUID)
   - title: String
   - content: String
   - created_at: DateTime<Utc>
   - updated_at: DateTime<Utc>

2. **シリアライゼーション対応**
   - Serdeのderiveマクロ適用

### Step 3: ファイルストレージシステムの実装

1. **FileStorage構造体の実装** (`src/storage/file_storage.rs`)
   - データディレクトリのパス管理
   - JSON形式での保存・読み込み

2. **CRUD操作の実装**
   - `save_article()`: 記事保存
   - `load_article()`: 記事読み込み
   - `list_articles()`: 記事一覧取得
   - `delete_article()`: 記事削除

### Step 4: CLIインターフェースの実装

1. **コマンド定義** (`src/cli/commands.rs`)
   - `create`: 記事作成
   - `list`: 記事一覧表示
   - `show`: 記事詳細表示
   - `edit`: 記事編集
   - `delete`: 記事削除

2. **Clapを使用したCLI引数解析**
   - サブコマンドの定義
   - 各種オプションの設定

### Step 5: メイン処理の実装

1. **main.rs**
   - CLI引数の解析
   - 各コマンドの実行
   - エラーハンドリング

2. **lib.rs**
   - モジュール構成の定義
   - 公開APIの設定

### Step 6: エラーハンドリングの強化

1. **カスタムエラー型の定義**
   - ファイルI/Oエラー
   - JSON解析エラー
   - 記事不存在エラー

2. **Result型の適切な使用**
   - エラーの伝播
   - エラーメッセージの改善

### Step 7: テストの実装

1. **ユニットテスト**
   - 記事モデルのテスト
   - ストレージ機能のテスト

2. **統合テスト**
   - CLI操作のテスト
   - ファイル操作のテスト

### Step 8: 最終調整

1. **コード品質チェック**
   - `cargo clippy`での警告解消
   - `cargo fmt`でのフォーマット

2. **動作確認**
   - 各コマンドの動作テスト
   - エラーケースの確認

## 実装順序の理由

1. **ボトムアップアプローチ**: 基礎となるデータ構造から実装
2. **依存関係を考慮**: 下位レイヤーから上位レイヤーへ
3. **段階的な機能追加**: 基本機能→エラーハンドリング→テスト

## 各ステップの成果物

- Step 1: プロジェクト構造、Cargo.toml
- Step 2: models/article.rs
- Step 3: storage/file_storage.rs
- Step 4: cli/commands.rs
- Step 5: main.rs, lib.rs
- Step 6: エラー型定義、Result使用
- Step 7: テストファイル群
- Step 8: 完全に動作するブログアプリ

## 確認ポイント

各ステップ完了時に以下を確認：
- [ ] コンパイルエラーがない
- [ ] 想定通りの動作をする
- [ ] エラーケースが適切に処理される
- [ ] コードが整理されている