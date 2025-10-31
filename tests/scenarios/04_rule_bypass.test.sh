#!/bin/bash
set -e

. "$(dirname "$0")/../helpers.sh"

TEST_URL_BYPASS="https://www.microsoft.com"

print_header "SCENARIO 04: Rule Engine (BYPASS)"
capture_initial_stats

print_step "First request (should BYPASS cache)"
assert_success "Request completed" run_proxied_curl ${TEST_URL_BYPASS} -o ${OUTPUT_FILE}
assert_stats_increment 0 0 "after first bypass request"

print_step "Second request (should also BYPASS cache)"
assert_success "Request completed" run_proxied_curl ${TEST_URL_BYPASS} -o ${OUTPUT_FILE}
assert_stats_increment 0 0 "after second bypass request"