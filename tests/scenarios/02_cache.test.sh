#!/bin/bash
set -e

. "$(dirname "$0")/../helpers.sh"

TEST_URL_CACHE="http://cachefly.cachefly.net/10mb.test"

print_header "SCENARIO 02: Cache (MISS and HIT)"
capture_initial_stats

print_step "First request (should be a CACHE MISS)"
assert_success "Request completed" run_proxied_curl ${TEST_URL_CACHE} -o ${OUTPUT_FILE}
assert_stats_increment 0 1 "after cache miss"

print_step "Second request (should be a CACHE HIT)"
assert_success "Request completed" run_proxied_curl ${TEST_URL_CACHE} -o ${OUTPUT_FILE}
assert_stats_increment 1 1 "after cache hit"