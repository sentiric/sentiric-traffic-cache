#!/bin/bash
set -e
set -x
    
. "$(dirname "$0")/../helpers.sh"

print_header "Testing Smart DNS Server"
    
dig_output=$(dig @${APP_HOST} example.com)
echo "${dig_output}"
if ! echo "${dig_output}" | grep -E "example\.com\.\s+60\s+IN\s+A\s+127\.0\.0\.1"; then
    echo "--- FAILURE: DNS server did not respond correctly. ---"
    exit 1
fi