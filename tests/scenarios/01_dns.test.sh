#!/bin/bash
set -e
    
. "$(dirname "$0")/../helpers.sh"

print_header "SCENARIO 01: Smart DNS Server"

if dig @${APP_HOST} example.com | grep -qE 'example\.com\.\s+60\s+IN\s+A\s+127\.0\.0\.1'; then
    echo "✅ SUCCESS: DNS server responded with the correct IP"
else
    echo "❌ FAILURE: DNS server did not respond correctly"
    exit 1
fi