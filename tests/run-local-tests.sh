#!/bin/bash
# BU SCRIPT, KENDİSİYLE AYNI DİZİNDE BULUNAN
# 'docker-compose.integration.yml' DOSYASINI ÇALIŞTIRMAK İÇİN TASARLANMIŞTIR.

set -e

# Script'in bulunduğu gerçek dizinin mutlak yolunu al.
# Bu sayede script'i nereden çalıştırdığınızın bir önemi kalmaz.
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
COMPOSE_FILE="$SCRIPT_DIR/docker-compose.integration.yml"

# 'trap' komutunun çalışabilmesi için cleanup fonksiyonunu önce tanımla
function cleanup() {
    echo "--- Cleaning up test environment ---"
    docker compose -f "$COMPOSE_FILE" down --remove-orphans --volumes
}

# Script bittiğinde (başarılı veya başarısız) cleanup fonksiyonunu çağır
trap cleanup EXIT

echo "--- Building and starting test environment from '$SCRIPT_DIR' ---"
# --force-recreate: Önceki çalıştırmadan kalan konteynerler varsa bile
#                   temiz bir başlangıç yapmayı garantiler.
docker compose -f "$COMPOSE_FILE" up --build --force-recreate --abort-on-container-exit --exit-code-from test-runner

echo "--- Test run finished. See exit code above. ---"