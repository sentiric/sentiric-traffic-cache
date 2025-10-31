#!/bin/bash
# Bu script, testleri YEREL MAKİNEDEN ORKESTRE EDER.

set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
COMPOSE_FILE="$SCRIPT_DIR/docker-compose.integration.yml"
TEST_RUNNER_SERVICE="test-runner" # <-- DOĞRU SERVİS ADI

# Temizlik fonksiyonunu başta tanımla
function cleanup() {
    echo "--- Cleaning up test environment ---"
    docker compose -f "$COMPOSE_FILE" down --remove-orphans --volumes
}

# Script bittiğinde (başarılı veya başarısız) cleanup fonksiyonunu çağır
trap cleanup EXIT

echo "--- Building and starting test environment from '$SCRIPT_DIR' ---"
docker compose -f "$COMPOSE_FILE" up --build -d

echo "--- Waiting for test-runner container to be in 'running' state ---"
TIMEOUT=30
SECONDS_WAITED=0
while true; do
    STATE=$(docker compose -f "$COMPOSE_FILE" ps --format '{{.State}}' "$TEST_RUNNER_SERVICE" 2>/dev/null || echo "not_found")
    
    echo "Current state of '$TEST_RUNNER_SERVICE' service: '$STATE'"

    if [ "$STATE" == "running" ]; then
        echo "✅ --- Test-runner container is running. Proceeding... ---"
        break
    fi

    if [ $SECONDS_WAITED -ge $TIMEOUT ]; then
        echo "❌ ERROR: Timed out after $TIMEOUT seconds waiting for '$TEST_RUNNER_SERVICE' to start."
        echo "Container logs:"
        docker compose -f "$COMPOSE_FILE" logs "$TEST_RUNNER_SERVICE"
        exit 1
    fi
    
    sleep 1
    SECONDS_WAITED=$((SECONDS_WAITED + 1))
done

find "${SCRIPT_DIR}/scenarios" -type f -name "*.test.sh" | sort | while read -r test_file; do
    TEST_NAME=$(basename "$test_file")
    echo ""
    echo "================================================="
    echo ">> RUNNING: ${TEST_NAME}"
    echo "================================================="

    echo "--- Resetting application state (clearing cache) ---"
    # GİRDİ YÖNLENDİRMESİ EKLE: < /dev/null
    docker compose -f "$COMPOSE_FILE" exec -T app curl -s -X POST http://localhost:8080/api/clear > /dev/null < /dev/null
    sleep 1

    # GİRDİ YÖNLENDİRMESİ EKLE: < /dev/null
    docker compose -f "$COMPOSE_FILE" exec -T "$TEST_RUNNER_SERVICE" "/tests/scenarios/${TEST_NAME}" < /dev/null
done

echo ""
echo "✅ --- ALL TEST SUITES PASSED ---"