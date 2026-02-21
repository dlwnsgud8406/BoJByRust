#!/bin/bash
# 테스트 통과 후 자동 commit & push
# Usage: rust_test_push.sh <problem_dir> <source_file>

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WORK_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$WORK_DIR"

PROBLEM_DIR="$1"
SOURCE_FILE="$2"

if [ -z "$PROBLEM_DIR" ] || [ -z "$SOURCE_FILE" ]; then
    echo "Usage: rust_test_push.sh <problem_dir> <source_file>"
    exit 1
fi

# 해당 문제 폴더의 변경된 파일 add
git add "$PROBLEM_DIR"/*.rs "$PROBLEM_DIR"/input*.txt "$PROBLEM_DIR"/output*.txt 2>/dev/null || true
git add "$PROBLEM_DIR"/*.cpp "$PROBLEM_DIR"/*.py 2>/dev/null || true

# 변경사항 있으면 commit & push
if ! git diff --cached --quiet 2>/dev/null; then
    PROBLEM_NAME=$(basename "$PROBLEM_DIR")
    git commit -m "solve: $PROBLEM_NAME ($SOURCE_FILE)"
    git push origin main
    echo "✅ 커밋 & 푸시 완료: $PROBLEM_NAME"
else
    echo "ℹ️  변경사항 없음"
fi

