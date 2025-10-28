#!/bin/bash
set -e
set -x

APP_HOST="app"
PROXY_URL="http://${APP_HOST}:3128"
MGMT_URL="http://${APP_HOST}:8080"
TEST_URL="http://cachefly.cachefly.net/10mb.test"
OUTPUT_FILE="/dev/null"

echo "--- Waiting for management API to be ready ---"

# Daha sağlam bir bekleme döngüsü
wait_for_api() {
    for i in {1..30}; do
        # -v ile detaylı çıktı al, -f ile hata kodu bekle, -s ile ilerleme çubuğunu gizle
        # 2>&1 ile hem stdout hem stderr'i birleştirip grep'e gönder
        if curl -v -s -f "${MGMT_URL}/api/stats" >/dev/null 2>&1; then
            echo "--- API is ready! ---"
            return 0
        fi
        echo "Attempt $i: Waiting for API..."
        # Hata ayıklama için app konteynerinin loglarını gösterelim
        echo "--- APP CONTAINER LOGS ---"
        docker logs sentiric-app-test || echo "Could not get app logs."
        echo "--------------------------"
        sleep 1
    done
    echo "--- FAILURE: API did not become ready in 30 seconds. ---"
    return 1
}

# Bekleme fonksiyonunu çağır
wait_for_api

echo "--- API is ready. Starting E2E tests. ---"

# --- Test 1: Cache Miss Senaryosu ---
echo "--- Running Cache MISS test ---"
curl -s -L --proxy ${PROXY_URL} ${TEST_URL} -o ${OUTPUT_FILE}

misses=$(curl -s ${MGMT_URL}/api/stats | jq '.misses')
if [ "${misses}" -ne 1 ]; then
    echo "--- FAILURE: Expected 1 miss, but got ${misses} ---"
    exit 1
fi
echo "--- SUCCESS: Cache MISS validated. ---"

# --- Test 2: Cache Hit Senaryosu ---
echo "--- Running Cache HIT test ---"
curl -s -L --proxy ${PROXY_URL} ${TEST_URL} -o ${OUTPUT_FILE}

hits=$(curl -s ${MGMT_URL}/api/stats | jq '.hits')
if [ "${hits}" -ne 1 ]; then
    echo "--- FAILURE: Expected 1 hit, but got ${hits} ---"
    exit 1
fi
echo "--- SUCCESS: Cache HIT validated. ---"

echo "--- ALL END-TO-END TESTS PASSED ---"
exit 0