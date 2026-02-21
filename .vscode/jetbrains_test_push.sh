#!/bin/bash
# JetBrains에서 테스트 통과 후 자동 commit & push
# Usage: jetbrains_test_push.sh <problem_dir>
# 예시: jetbrains_test_push.sh "1000. A+B"

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WORK_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$WORK_DIR"

PROBLEM_DIR="$1"

if [ -z "$PROBLEM_DIR" ]; then
    echo "Usage: jetbrains_test_push.sh <problem_dir>"
    exit 1
fi

# ── 1. 빌드 ──────────────────────────────────────────────
SOURCE_RS=$(ls "$PROBLEM_DIR"/*.rs 2>/dev/null | head -1)
if [ -z "$SOURCE_RS" ]; then
    echo "❌ .rs 파일을 찾을 수 없습니다: $PROBLEM_DIR"
    exit 1
fi

BINARY="${PROBLEM_DIR}/$(basename "$SOURCE_RS" .rs)"
echo "🔨 빌드 중: $SOURCE_RS"
rustc "$SOURCE_RS" -o "$BINARY"

# ── 2. 테스트 ─────────────────────────────────────────────
echo "🧪 테스트 실행 중..."
bash "$SCRIPT_DIR/test.sh" "$BINARY"
TEST_RESULT=$?

# ── 3. 테스트 통과 시 commit & push ───────────────────────
if [ $TEST_RESULT -eq 0 ]; then
    echo ""
    echo "✅ 테스트 통과! GitHub에 push합니다..."

    # 변경 파일 스테이징
    git add "$PROBLEM_DIR"/*.rs 2>/dev/null || true
    git add "$PROBLEM_DIR"/input*.txt "$PROBLEM_DIR"/output*.txt 2>/dev/null || true
    git add "$PROBLEM_DIR"/*.cpp "$PROBLEM_DIR"/*.py 2>/dev/null || true

    # 변경사항이 있을 때만 commit & push
    if ! git diff --cached --quiet 2>/dev/null; then
        PROBLEM_NAME=$(basename "$PROBLEM_DIR")
        SOURCE_NAME=$(basename "$SOURCE_RS")
        git commit -m "solve: $PROBLEM_NAME ($SOURCE_NAME)"
        git push origin main
        echo "🚀 커밋 & 푸시 완료: $PROBLEM_NAME"
    else
        echo "ℹ️  스테이징된 변경사항 없음 (이미 커밋됨)"
    fi
else
    echo ""
    echo "❌ 테스트 실패 - push 하지 않습니다."
    exit 1
fi

