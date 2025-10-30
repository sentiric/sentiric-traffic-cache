#!/bin/bash
set -e

. "$(dirname "$0")/../helpers.sh"

TEST_URL_CACHE="http://cachefly.cachefly.net/10mb.test"

print_header "Testing Cache (MISS and HIT)"
capture_initial_stats

echo "--- Step 1: Cache MISS ---"
run_proxied_curl ${TEST_URL_CACHE} -o ${OUTPUT_FILE}
assert_stats_increment 0 1 "after cache miss"

echo "--- Step 2: Cache HIT ---"
run_proxied_curl ${TEST_URL_CACHE} -o ${OUTPUT_FILE}
assert_stats_increment 1 1 "after cache hit"