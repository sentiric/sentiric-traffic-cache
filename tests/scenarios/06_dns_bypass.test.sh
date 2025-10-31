#!/bin/bash
set -e

. "$(dirname "$0")/../helpers.sh"

TEST_URL_BYPASS="https://www.microsoft.com"

print_header "SCENARIO 06: DNS End-to-End (BYPASS)"
capture_initial_stats

print_step "First request via DNS (should BYPASS cache)"
assert_success "Request completed" run_direct_curl ${TEST_URL_BYPASS} -o ${OUTPUT_FILE}
assert_stats_increment 0 0 "after first DNS bypass request"

print_step "Second request via DNS (should also BYPASS cache)"
assert_success "Request completed" run_direct_curl ${TEST_URL_BYPASS} -o ${OUTPUT_FILE}
assert_stats_increment 0 0 "after second DNS bypass request"