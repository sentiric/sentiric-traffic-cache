#!/bin-bash
set -e

. "$(dirname "$0")/../helpers.sh"

TEST_URL_BYPASS="https://www.microsoft.com"

print_header "Testing Smart DNS End-to-End: BYPASS"
capture_initial_stats

# Bu test, proxy ortam değişkenleri OLMADAN çalıştırılır.
# İsteğin başarılı olması VE önbellek istatistiklerini DEĞİŞTİRMEMESİ,
# DNS tabanlı şeffaf proxy'nin 'bypass' kuralını doğru uyguladığını kanıtlar.
echo "--- Step 1: First request via DNS (should NOT affect cache stats) ---"
run_direct_curl ${TEST_URL_BYPASS} -o ${OUTPUT_FILE}
assert_stats_increment 0 0 "after first DNS bypass request"

echo "--- Step 2: Second request via DNS (should ALSO NOT affect cache stats) ---"
run_direct_curl ${TEST_URL_BYPASS} -o ${OUTPUT_FILE}
assert_stats_increment 0 0 "after second DNS bypass request"

echo "--- SUCCESS: DNS transparent proxy correctly bypassed the cache. ---"