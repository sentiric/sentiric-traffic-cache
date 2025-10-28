#!/bin/bash
set -e
set -x

APP_HOST="app"
PROXY_URL="http://${APP_HOST}:3128"
MGMT_URL="http://${APP_HOST}:8080"
TEST_URL="http://cachefly.cachefly.net/10mb.test"
OUTPUT_FILE="/dev/null"

echo "--- Services are starting. Waiting for API to be healthy. ---"
# docker-compose.yml'deki healthcheck bu bekleme işini yapıyor.

echo "--- Services are healthy. Starting E2E tests. ---"

# --- Test 0: DNS Sunucusunu Test Et ---
echo "--- Running DNS server test ---"
# 'dig' komutu ile 'google.com' adresini 'app' konteynerine sor
# Cevabın '127.0.0.1' içerdiğini doğrula
dig_output=$(dig @${APP_HOST} google.com)
echo "${dig_output}" # Hata ayıklama için çıktıyı göster

if ! echo "${dig_output}" | grep -E "google\.com\.\s+60\s+IN\s+A\s+127\.0\.0\.1"; then
    echo "--- FAILURE: DNS server did not respond with 127.0.0.1! ---"
    exit 1
fi
echo "--- SUCCESS: Smart DNS server validated. ---"


# Testten önce istatistiklerin sıfır olduğundan emin olalım.
initial_stats=$(curl -s ${MGMT_URL}/api/stats)
initial_hits=$(echo ${initial_stats} | jq '.hits')
initial_misses=$(echo ${initial_stats} | jq '.misses')

if [ "${initial_hits}" -ne 0 ] || [ "${initial_misses}" -ne 0 ]; then
    echo "--- FAILURE: Initial stats are not zero! Hits: ${initial_hits}, Misses: ${initial_misses} ---"
    exit 1
fi

# ... (geri kalan testler aynı)
echo "--- Running Cache MISS test ---"
curl -s -L --proxy ${PROXY_URL} ${TEST_URL} -o ${OUTPUT_FILE}
stats_after_miss=$(curl -s ${MGMT_URL}/api/stats)
misses=$(echo ${stats_after_miss} | jq '.misses')
hits=$(echo ${stats_after_miss} | jq '.hits')
if [ "${misses}" -ne 1 ]; then
    echo "--- FAILURE: Expected 1 miss, but got ${misses} ---"
    exit 1
fi
if [ "${hits}" -ne 0 ]; then
    echo "--- FAILURE: Expected 0 hits after miss, but got ${hits} ---"
    exit 1
fi
echo "--- SUCCESS: Cache MISS validated. ---"

echo "--- Running Cache HIT test ---"
curl -s -L --proxy ${PROXY_URL} ${TEST_URL} -o ${OUTPUT_FILE}
stats_after_hit=$(curl -s ${MGMT_URL}/api/stats)
misses=$(echo ${stats_after_hit} | jq '.misses')
hits=$(echo ${stats_after_hit} | jq '.hits')
if [ "${misses}" -ne 1 ]; then
    echo "--- FAILURE: Expected misses to remain 1, but got ${misses} ---"
    exit 1
fi
if [ "${hits}" -ne 1 ]; then
    echo "--- FAILURE: Expected 1 hit, but got ${hits} ---"
    exit 1
fi
echo "--- SUCCESS: Cache HIT validated. ---"

echo "--- ALL END-TO-END TESTS PASSED ---"
exit 0