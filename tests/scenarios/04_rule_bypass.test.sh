#!/bin/bash
set -e
set -x

. "$(dirname "$0")/../helpers.sh"

TEST_URL_BYPASS="https://www.microsoft.com"

print_header "Testing Rule Engine: BYPASS"
clear_stats

echo "--- Step 1: First request (should be a miss) ---"
curl -s -L --proxy ${PROXY_URL} -k ${TEST_URL_BYPASS} -o ${OUTPUT_FILE}
assert_stats 0 1 "after first bypass request"

echo "--- Step 2: Second request (should also be a miss) ---"
curl -s -L --proxy ${PROXY_URL} -k ${TEST_URL_BYPASS} -o ${OUTPUT_FILE}
assert_stats 0 2 "after second bypass request"