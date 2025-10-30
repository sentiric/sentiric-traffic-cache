#!/bin/bash
set -e
set -x

. "$(dirname "$0")/../helpers.sh"

TEST_URL_CACHE="http://cachefly.cachefly.net/10mb.test"

print_header "Testing Cache (MISS and HIT)"
clear_stats

echo "--- Step 1: Cache MISS ---"
curl -s -L --proxy ${PROXY_URL} ${TEST_URL_CACHE} -o ${OUTPUT_FILE}
assert_stats 0 1 "after cache miss"

echo "--- Step 2: Cache HIT ---"
curl -s -L --proxy ${PROXY_URL} ${TEST_URL_CACHE} -o ${OUTPUT_FILE}
assert_stats 1 1 "after cache hit"