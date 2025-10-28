#!/bin/bash
set -e # Herhangi bir komut hata verirse script'i durdur
set -x # Çalışan komutları ekrana yaz

# Değişkenler
APP_HOST="app"
PROXY_URL="http://${APP_HOST}:3128"
MGMT_URL="http://${APP_HOST}:8080"
TEST_URL="http://cachefly.cachefly.net/10mb.test"
OUTPUT_FILE="/dev/null"

echo "--- Waiting for management API to be ready ---"
# API'nin cevap vermesini bekle (maksimum 30 saniye)
timeout 30s bash -c 'until curl -s -f ${MGMT_URL}/api/stats > /dev/null; do echo "Waiting for API..." && sleep 1; done'

echo "--- API is ready. Starting E2E tests. ---"

# --- Test 1: Cache Miss Senaryosu ---
echo "--- Running Cache MISS test ---"
# Proxy üzerinden ilk isteği gönder
curl -s -L --proxy ${PROXY_URL} ${TEST_URL} -o ${OUTPUT_FILE}

# API'den istatistikleri al ve 'misses' sayacını kontrol et
misses=$(curl -s ${MGMT_URL}/api/stats | jq '.misses')
if [ "${misses}" -ne 1 ]; then
    echo "--- FAILURE: Expected 1 miss, but got ${misses} ---"
    exit 1
fi
echo "--- SUCCESS: Cache MISS validated. ---"


# --- Test 2: Cache Hit Senaryosu ---
echo "--- Running Cache HIT test ---"
# Proxy üzerinden ikinci isteği gönder
curl -s -L --proxy ${PROXY_URL} ${TEST_URL} -o ${OUTPUT_FILE}

# API'den istatistikleri al ve 'hits' sayacını kontrol et
hits=$(curl -s ${MGMT_URL}/api/stats | jq '.hits')
if [ "${hits}" -ne 1 ]; then
    echo "--- FAILURE: Expected 1 hit, but got ${hits} ---"
    exit 1
fi
echo "--- SUCCESS: Cache HIT validated. ---"


# --- Sonuç ---
echo "--- ALL END-TO-END TESTS PASSED ---"
exit 0