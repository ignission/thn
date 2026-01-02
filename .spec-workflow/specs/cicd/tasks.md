# Tasks Document: CI/CD

- [x] 1. CIワークフロー作成
  - File: `.github/workflows/ci.yml`
  - PR/mainプッシュ時にfmt, clippy, testを実行
  - Swatinem/rust-cacheでキャッシュ設定
  - Purpose: コード品質チェックの自動化

- [x] 2. CDワークフロー作成（ビルド部分）
  - File: `.github/workflows/release.yml`
  - タグv*プッシュ時にマトリックスビルドを実行
  - 5ターゲット: macOS(x86_64, aarch64), Linux(x86_64, aarch64), Windows(x86_64)
  - Purpose: クロスプラットフォームバイナリの自動ビルド

- [x] 3. CDワークフロー作成（リリース部分）
  - File: `.github/workflows/release.yml`（タスク2に追加）
  - ビルド完了後にGitHub Releaseを作成
  - バイナリをtar.gz/zipでアーカイブして添付
  - Purpose: GitHub Releasesへの自動公開

- [x] 4. Homebrew Formula自動更新
  - File: `.github/workflows/release.yml`（タスク3に追加）
  - リリース後にhomebrew-tapリポジトリのFormulaを更新
  - SHA256ハッシュを計算してFormula更新
  - Purpose: Homebrew配布の自動化

- [x] 5. READMEにCIバッジ追加
  - File: `README.md`
  - CIステータスバッジを追加
  - Purpose: プロジェクト品質の可視化
