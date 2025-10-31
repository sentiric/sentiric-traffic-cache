#!/bin/bash
set -e

. "$(dirname "$0")/../helpers.sh"

TEST_URL_BYPASS="https://www.microsoft.com"

print_header "Testing Rule Engine: BYPASS"
capture_initial_stats

# Sürdürülebilirlik Notu:
# Bir 'bypass-cache' isteği, önbelleği tamamen yok sayar.
# Bu nedenle ne bir 'hit' ne de bir 'miss' olarak sayılmalıdır.
# Doğru beklenti, önbellek istatistiklerinin (hits, misses) HİÇ değişmemesidir.

echo "--- Step 1: First request (should NOT affect cache stats) ---"
run_proxied_curl ${TEST_URL_BYPASS} -o ${OUTPUT_FILE}
assert_stats_increment 0 0 "after first bypass request"

echo "--- Step 2: Second request (should ALSO NOT affect cache stats) ---"
run_proxied_curl ${TEST_URL_BYPASS} -o ${OUTPUT_FILE}
assert_stats_increment 0 0 "after second bypass request"

echo "--- SUCCESS: Bypass rule correctly ignored the cache on multiple requests. ---"