#!/bin/bash
set -e

. "$(dirname "$0")/../helpers.sh"

TEST_URL_BYPASS="https://www.microsoft.com"

print_header "Testing Rule Engine: BYPASS"
capture_initial_stats

echo "--- Step 1: First request (should be a miss) ---"
run_proxied_curl ${TEST_URL_BYPASS} -o ${OUTPUT_FILE}
assert_stats_increment 0 1 "after first bypass request"

echo "--- Step 2: Second request (should also be a miss) ---"
run_proxied_curl ${TEST_URL_BYPASS} -o ${OUTPUT_FILE}
assert_stats_increment 0 2 "after second bypass request"