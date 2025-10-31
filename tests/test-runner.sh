#!/bin/bash
set -e # Bir test başarısız olursa tüm süreci durdur

BASE_DIR=$(dirname "$0")
TEST_RUNNER_CONTAINER="sentiric-tester-runner"

# Önbelleği ve durumu her testten ÖNCE temizlemek için bir fonksiyon
function reset_app_state() {
    echo "--- Resetting application state (clearing cache) ---"
    docker compose -f "${BASE_DIR}/docker-compose.integration.yml" exec -T app \
        curl -s -X POST http://localhost:8080/api/clear > /dev/null
    sleep 1 # Servisin toparlanması için kısa bir bekleme
}


# Eğer script'e bir argüman verildiyse, sadece o testi çalıştır.
if [ -n "$1" ]; then
    if [[ "$1" == *".test.sh" ]]; then
        echo "--- Running single test: $1 ---"
        reset_app_state
        docker compose -f "${BASE_DIR}/docker-compose.integration.yml" exec -T "$TEST_RUNNER_CONTAINER" "/tests/scenarios/$(basename "$1")"
        exit 0
    else
        echo "--- ERROR: Test file must be a .test.sh file ---"
        exit 1
    fi
fi

# Eğer argüman yoksa, tüm testleri sırayla çalıştır.
echo "--- Running All Integration Test Suites ---"
find "${BASE_DIR}/scenarios" -type f -name "*.test.sh" | sort | while read -r test_file; do
    reset_app_state
    # Her test script'ini kendi izole `exec` komutu içinde çalıştır
    docker compose -f "${BASE_DIR}/docker-compose.integration.yml" exec -T "$TEST_RUNNER_CONTAINER" "/tests/scenarios/$(basename "$test_file")"
done

echo ""
echo "--- ALL TEST SUITES PASSED ---"