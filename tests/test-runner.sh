#!/bin/bash
set -e
set -x

APP_HOST="app"
PROXY_URL="http://${APP_HOST}:3128"
MGMT_URL="http://${APP_HOST}:8080"
TEST_URL="http://cachefly.cachefly.net/10mb.test"
OUTPUT_FILE="/dev/null"

# API'nin hazır olmasını bekleme fonksiyonu (artık depends_on:service_healthy ile gerek yok ama
# yine de bir güvenlik önlemi olarak kalabilir)
echo "--- Waiting for services to be healthy ---"
# docker-compose.yml'deki healthcheck bu bekleme işini zaten yapıyor.
# Buraya sadece bir log mesajı koyuyoruz.

echo "--- Services are healthy. Starting E2E tests. ---"

# Testten önce istatistiklerin sıfır olduğundan emin olalım.
# Başlangıçtaki API kontrolü bile sayaçları etkileyebilir.
initial_stats=$(curl -s ${MGMT_URL}/api/stats)
initial_hits=$(echo ${initial_stats} | jq '.hits')
initial_misses=$(echo ${initial_stats} | jq '.misses')

if [ "${initial_hits}" -ne 0 ] || [ "${initial_misses}" -ne 0 ]; then
    echo "--- FAILURE: Initial stats are not zero! Hits: ${initial_hits}, Misses: ${initial_misses} ---"
    exit 1
fi

# --- Test 1: Cache Miss Senaryosu ---
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


# --- Test 2: Cache Hit Senaryosu ---
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

