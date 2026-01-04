#!/bin/bash
set -e

# 引数チェック
if [ -z "$1" ]; then
    echo "Usage: $0 <version> [release_notes]"
    echo "Example: $0 0.4.0 'New feature release'"
    exit 1
fi

VERSION="$1"
RELEASE_NOTES="${2:-}"

# バージョン形式チェック
if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
    echo "error: invalid version format. Use semantic versioning (e.g., 0.4.0)"
    exit 1
fi

# 作業ディレクトリがクリーンか確認
if [ -n "$(git status --porcelain)" ]; then
    echo "error: working directory is not clean. Commit or stash changes first."
    exit 1
fi

# mainブランチか確認
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo "error: must be on main branch (currently on $CURRENT_BRANCH)"
    exit 1
fi

# 最新の状態か確認
git fetch origin
if [ "$(git rev-parse HEAD)" != "$(git rev-parse origin/main)" ]; then
    echo "error: local main is not up to date with origin/main"
    exit 1
fi

echo "==> Updating version to $VERSION"

# Cargo.toml のバージョンを更新
sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

# ビルドして Cargo.lock を更新
echo "==> Building to update Cargo.lock"
cargo build --release

# テスト実行
echo "==> Running tests"
cargo test

# コミット（Cargo.toml と Cargo.lock を一緒に）
echo "==> Committing version bump"
git add Cargo.toml Cargo.lock
git commit -m "chore: v$VERSION"

# タグ作成
echo "==> Creating tag v$VERSION"
git tag "v$VERSION"

# プッシュ
echo "==> Pushing to origin"
git push origin main
git push origin "v$VERSION"

# リリース作成
echo "==> Creating GitHub release"
if [ -n "$RELEASE_NOTES" ]; then
    gh release create "v$VERSION" --title "v$VERSION" --notes "$RELEASE_NOTES"
else
    gh release create "v$VERSION" --title "v$VERSION" --generate-notes
fi

echo ""
echo "✓ Released v$VERSION successfully!"
echo "  https://github.com/ignission/thn/releases/tag/v$VERSION"
