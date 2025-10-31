#!/bin/bash
set -e

# Script'in bulunduğu dizini al (nereden çağrılırsa çağrılsın)
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
COMPOSE_FILE="$SCRIPT_DIR/tests/docker-compose.integration.yml"

# 'trap' komutunun çalışabilmesi için cleanup fonksiyonunu önce tanımla
function cleanup() {
    echo "--- Cleaning up test environment ---"
    docker compose -f "$COMPOSE_FILE" down --remove-orphans --volumes
}

# Script bittiğinde (başarılı veya başarısız) cleanup fonksiyonunu çağır
trap cleanup EXIT

echo "--- Building and starting test environment ---"
docker compose -f "$COMPOSE_FILE" up --build --abort-on-container-exit --exit-code-from test-runner

echo "--- Test run finished. See exit code above. ---"