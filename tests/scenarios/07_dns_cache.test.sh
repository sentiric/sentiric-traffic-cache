#!/bin/bash
# Bu script, DNS tabanlı şeffaf proxy üzerinden gelen bir HTTPS isteğinin
# ilk seferde önbelleğe alınıp (MISS), ikinci seferde önbellekten (HIT)
# sunulduğunu doğrular.

set -e
set -x

. "$(dirname "$0")/../helpers.sh"

# Kurallarda block veya bypass edilmeyen, stabil ve güvenilir bir HTTPS URL'i seçiyoruz.
TEST_URL_HTTPS_CACHE="https://raw.githubusercontent.com/httpie/httpie/master/README.md"

print_header "Testing Cache over Smart DNS (HTTPS End-to-End)"
capture_initial_stats

echo "--- Step 1: Cache MISS (via DNS) ---"
# Proxy ayarı olmadan, sadece --dns yönlendirmesiyle curl çalıştırıyoruz.
run_direct_curl ${TEST_URL_HTTPS_CACHE} -o ${OUTPUT_FILE}
assert_stats_increment 0 1 "after DNS cache miss"

echo "--- Step 2: Cache HIT (via DNS) ---"
# Aynı isteği tekrar yapıyoruz.
run_direct_curl ${TEST_URL_HTTPS_CACHE} -o ${OUTPUT_FILE}
assert_stats_increment 1 1 "after DNS cache hit"

echo "--- SUCCESS: Caching for HTTPS traffic over DNS is working correctly. ---"