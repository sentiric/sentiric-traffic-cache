#!/bin/bash
set -e
set -x

APP_HOST="app"
PROXY_URL="http://${APP_HOST}:3128"
MGMT_URL="http://${APP_HOST}:8080"
TEST_URL_CACHE="http://cachefly.cachefly.net/10mb.test"
TEST_URL_BLOCK="https://www.google.com"
TEST_URL_BYPASS="https://www.microsoft.com"
OUTPUT_FILE="/dev/null"

echo "--- Services are healthy. Starting E2E tests. ---"

# --- Test 0: DNS Sunucusunu Test Et ---
echo "--- Running DNS server test ---"
dig_output=$(dig @${APP_HOST} example.com)
echo "${dig_output}"
if ! echo "${dig_output}" | grep -E "example\.com\.\s+60\s+IN\s+A\s+127\.0\.0\.1"; then
    echo "--- FAILURE: DNS server did not respond with 127.0.0.1! ---"
    exit 1
fi
echo "--- SUCCESS: Smart DNS server validated. ---"

# --- Test 1: Cache MISS ve HIT ---
echo "--- Clearing cache before test ---"
curl -s -X POST ${MGMT_URL}/api/clear
initial_stats=$(curl -s ${MGMT_URL}/api/stats)
initial_hits=$(echo ${initial_stats} | jq '.hits')
initial_misses=$(echo ${initial_stats} | jq '.misses')
if [ "${initial_hits}" -ne 0 ] || [ "${initial_misses}" -ne 0 ]; then
    echo "--- FAILURE: Initial stats are not zero after clear! ---"
    exit 1
fi

echo "--- Running Cache MISS test ---"
curl -s -L --proxy ${PROXY_URL} ${TEST_URL_CACHE} -o ${OUTPUT_FILE}
stats_after_miss=$(curl -s ${MGMT_URL}/api/stats)
misses=$(echo ${stats_after_miss} | jq '.misses')
if [ "${misses}" -ne 1 ]; then
    echo "--- FAILURE: Expected 1 miss, but got ${misses} ---"
    exit 1
fi
echo "--- SUCCESS: Cache MISS validated. ---"

echo "--- Running Cache HIT test ---"
curl -s -L --proxy ${PROXY_URL} ${TEST_URL_CACHE} -o ${OUTPUT_FILE}
stats_after_hit=$(curl -s ${MGMT_URL}/api/stats)
hits=$(echo ${stats_after_hit} | jq '.hits')
if [ "${hits}" -ne 1 ]; then
    echo "--- FAILURE: Expected 1 hit, but got ${hits} ---"
    exit 1
fi
echo "--- SUCCESS: Cache HIT validated. ---"


# --- YENİ TESTLER ---

# --- Test 2: Kural Motoru - BLOCK ---
echo "--- Running Rule Engine BLOCK test ---"
# --fail: HTTP 4xx veya 5xx hatası alırsa script'i başarısız yapar
# -k: Sertifika doğrulamasını atlar (MitM yaptığımız için gerekli)
if curl -s -L --proxy ${PROXY_URL} -k ${TEST_URL_BLOCK} -o ${OUTPUT_FILE} --fail; then
    echo "--- FAILURE: Request to ${TEST_URL_BLOCK} was not blocked! ---"
    exit 1
fi
echo "--- SUCCESS: Rule Engine BLOCK validated. ---"


# --- Test 3: Kural Motoru - BYPASS ---
echo "--- Running Rule Engine BYPASS test ---"
# Önbelleği temizle ve bypass testine başla
curl -s -X POST ${MGMT_URL}/api/clear

# İlk istek (miss olmalı)
curl -s -L --proxy ${PROXY_URL} -k ${TEST_URL_BYPASS} -o ${OUTPUT_FILE}
stats_after_bypass1=$(curl -s ${MGMT_URL}/api/stats)
misses1=$(echo ${stats_after_bypass1} | jq '.misses')
hits1=$(echo ${stats_after_bypass1} | jq '.hits')

if [ "${misses1}" -ne 1 ] || [ "${hits1}" -ne 0 ]; then
    echo "--- FAILURE: Expected 1 miss and 0 hits after first bypass request! ---"
    exit 1
fi

# İkinci istek (yine miss olmalı, çünkü cache'e yazılmamalı)
curl -s -L --proxy ${PROXY_URL} -k ${TEST_URL_BYPASS} -o ${OUTPUT_FILE}
stats_after_bypass2=$(curl -s ${MGMT_URL}/api/stats)
misses2=$(echo ${stats_after_bypass2} | jq '.misses')
hits2=$(echo ${stats_after_bypass2} | jq '.hits')

if [ "${misses2}" -ne 2 ] || [ "${hits2}" -ne 0 ]; then
    echo "--- FAILURE: Expected 2 misses and 0 hits after second bypass request! Cache was not bypassed. ---"
    exit 1
fi
echo "--- SUCCESS: Rule Engine BYPASS validated. ---"


echo "--- ALL END-TO-END TESTS PASSED ---"
exit 0