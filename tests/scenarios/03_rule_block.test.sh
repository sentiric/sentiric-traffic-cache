#!/bin/bash
set -e
set -x

. "$(dirname "$0")/../helpers.sh"

TEST_URL_BLOCK="https://www.google.com"

print_header "Testing Rule Engine: BLOCK"

if curl -s -L --proxy ${PROXY_URL} -k ${TEST_URL_BLOCK} -o ${OUTPUT_FILE} --fail; then
    echo "--- FAILURE: Request to ${TEST_URL_BLOCK} was NOT blocked. ---"
    exit 1
else
    echo "--- SUCCESS: Request was correctly blocked. ---"
fi