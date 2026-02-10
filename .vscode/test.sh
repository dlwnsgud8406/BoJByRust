#!/bin/bash
# Usage: test.sh <command> [args...]
# 예시: test.sh ./2753
#       test.sh python3 2753.py

CMD=("$@")
passed=0
failed=0
total=0

for inp in input*.txt; do
    if [ -f "$inp" ]; then
        out="output${inp#input}"
        if [ -f "$out" ]; then
            total=$((total+1))
            echo "=== 테스트: $inp / $out ==="
            actual=$("${CMD[@]}" < "$inp")
            expected=$(cat "$out")
            if [ "$actual" = "$expected" ]; then
                echo '✅ 정답!'
                passed=$((passed+1))
            else
                echo '❌ 틀렸습니다.'
                diff <(echo "$actual") <(echo "$expected")
                failed=$((failed+1))
            fi
        fi
    fi
done

echo
echo "=== 결과: ${total}개 중 ${passed}개 정답, ${failed}개 오답 ==="

if [ "$failed" -gt 0 ]; then
    exit 1
fi
