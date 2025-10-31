#!/bin/bash
set -e

. "$(dirname "$0")/../helpers.sh"

TEST_URL_HTTPS_CACHE="https://raw.githubusercontent.com/httpie/httpie/master/README.md"

print_header "SCENARIO 07: DNS End-to-End (Cache MISS and HIT)"
capture_initial_stats

print_step "First request via DNS (should be a CACHE MISS)"
assert_success "Request completed" run_direct_curl ${TEST_URL_HTTPS_CACHE} -o ${OUTPUT_FILE}
assert_stats_increment 0 1 "after DNS cache miss"

print_step "Second request via DNS (should be a CACHE HIT)"
assert_success "Request completed" run_direct_curl ${TEST_URL_HTTPS_CACHE} -o ${OUTPUT_FILE}
assert_stats_increment 1 1 "after DNS cache hit"