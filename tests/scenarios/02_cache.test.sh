#!/bin/bash
set -e
set -x

. "$(dirname "$0")/../helpers.sh"

TEST_URL_CACHE="http://cachefly.cachefly.net/10mb.test"

print_header "Testing Cache (MISS and HIT)"
capture_initial_stats

echo "--- Step 1: Cache MISS ---"
curl -s -L --proxy ${PROXY_URL} ${TEST_URL_CACHE} -o ${OUTPUT_FILE}
# Beklenti: Miss sayısı 1 artacak, hit sayısı artmayacak.
assert_stats_increment 0 1 "after cache miss"

echo "--- Step 2: Cache HIT ---"
curl -s -L --proxy ${PROXY_URL} ${TEST_URL_CACHE} -o ${OUTPUT_FILE}
# Beklenti: Miss sayısı aynı kalacak, hit sayısı 1 artacak.
assert_stats_increment 1 1 "after cache hit"