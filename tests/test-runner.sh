#!/bin/bash
set -e

BASE_DIR=$(dirname "$0")

# Eğer script'e bir argüman verildiyse, sadece o testi çalıştır.
if [ -n "$1" ]; then
    if [ -f "$1" ]; then
        echo "--- Running single test: $1 ---"
        /bin/bash "$1"
        exit 0
    else
        echo "--- ERROR: Test file not found: $1 ---"
        exit 1
    fi
fi

# Eğer argüman yoksa, tüm testleri sırayla çalıştır.
echo "--- Running All Integration Test Suites ---"
find "${BASE_DIR}/scenarios" -type f -name "*.test.sh" -executable | sort | while read -r test_file; do
    /bin/bash "$test_file"
done

echo ""
echo "--- ALL TEST SUITES PASSED ---"