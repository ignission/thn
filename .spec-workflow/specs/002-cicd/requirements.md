# Requirements Document: CI/CD

## Introduction

thnプロジェクトにGitHub Actionsを使用したCI/CDパイプラインを導入する。プルリクエストとmainブランチへのプッシュ時に自動でテスト、リント、ビルドを実行し、リリースタグ作成時にはクロスプラットフォームのバイナリをビルドしてGitHub Releasesに公開する。

## Alignment with Product Vision

product.mdに記載された以下の目標を支援する:

- **信頼性**: 自動テストにより、メモが正しい位置に正しいフォーマットで追記されることを継続的に保証
- **Homebrewでの簡単インストール**: クロスプラットフォームビルドによりHomebrewやcargoでの配布を容易に
- **オープンソースコミュニティへの価値提供**: CIバッジによる品質可視化、コントリビューターへの迅速なフィードバック

## Requirements

### Requirement 1: 継続的インテグレーション（CI）

**User Story:** As a 開発者, I want プルリクエスト時に自動でテストとリントが実行される, so that コード品質を維持しながら安心してマージできる

#### Acceptance Criteria

1. WHEN プルリクエストが作成される THEN システム SHALL cargo testを実行し結果を報告する
2. WHEN プルリクエストが作成される THEN システム SHALL cargo fmt --checkを実行しフォーマット違反を検出する
3. WHEN プルリクエストが作成される THEN システム SHALL cargo clippyを実行し警告を検出する
4. WHEN mainブランチにプッシュされる THEN システム SHALL 同様のチェックを実行する
5. IF いずれかのチェックが失敗する THEN システム SHALL ワークフローを失敗ステータスで終了する

### Requirement 2: ビルドキャッシュ

**User Story:** As a 開発者, I want CIのビルド時間が短縮される, so that フィードバックを素早く得られる

#### Acceptance Criteria

1. WHEN CIが実行される THEN システム SHALL Cargo依存関係をキャッシュする
2. WHEN 依存関係に変更がない THEN システム SHALL キャッシュからリストアしてビルド時間を短縮する
3. WHEN Cargo.lockが変更される THEN システム SHALL キャッシュを更新する

### Requirement 3: クロスプラットフォームビルド

**User Story:** As a メンテナ, I want リリース時に複数プラットフォーム向けバイナリが自動生成される, so that 手動ビルドの手間なく配布できる

#### Acceptance Criteria

1. WHEN リリースタグ(v*)が作成される THEN システム SHALL macOS (x86_64, aarch64) 向けバイナリをビルドする
2. WHEN リリースタグ(v*)が作成される THEN システム SHALL Linux (x86_64, aarch64) 向けバイナリをビルドする
3. WHEN リリースタグ(v*)が作成される THEN システム SHALL Windows (x86_64) 向けバイナリをビルドする
4. WHEN 全プラットフォームのビルドが成功する THEN システム SHALL GitHub Releaseを作成しバイナリを添付する

### Requirement 4: MSRV（最小サポートRustバージョン）検証

**User Story:** As a 開発者, I want 最小サポートバージョンでのビルド確認ができる, so that 互換性を維持できる

#### Acceptance Criteria

1. WHEN プルリクエストが作成される THEN システム SHALL Rust 1.70（MSRV）でのビルドを検証する
2. IF MSRVでビルドが失敗する THEN システム SHALL 警告を報告する

## Non-Functional Requirements

### Code Architecture and Modularity
- **ワークフロー分離**: CI（テスト/リント）とCD（リリース）を別ワークフローに分離
- **再利用可能**: マトリックスビルドで重複を削減

### Performance
- キャッシュ活用によりCIビルド時間を3分以内に抑える
- リリースビルドは15分以内に完了する

### Security
- GitHub Actionsのpermissionsを最小限に設定
- サードパーティアクションはピン留めしたSHAを使用

### Reliability
- ワークフロー失敗時は明確なエラーメッセージを表示
- フレーキーテストを防ぐため、環境依存テストには適切なタイムアウトを設定

### Usability
- READMEにCIステータスバッジを追加
- ワークフロー名は日本語で分かりやすく
