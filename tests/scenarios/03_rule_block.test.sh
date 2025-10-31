#!/bin/bash
set -e

. "$(dirname "$0")/../helpers.sh"

TEST_URL_BLOCK="https://www.google.com"

print_header "SCENARIO 03: Rule Engine (BLOCK)"

assert_failure "Request to ${TEST_URL_BLOCK} was correctly blocked" \
    run_proxied_curl ${TEST_URL_BLOCK} -o ${OUTPUT_FILE} --fail