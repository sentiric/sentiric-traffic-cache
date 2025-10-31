#!/bin/bash
# Bu script, tüm entegrasyon testlerini CI ortamını birebir taklit ederek
# yerel makinede tek bir komutla çalıştırır.

# Herhangi bir komut başarısız olursa script'i anında durdur
set -e

COMPOSE_FILE="tests/docker-compose.integration.yml"

# Script bittiğinde (başarılı veya başarısız) cleanup fonksiyonunu çağır
trap cleanup EXIT

function cleanup() {
    echo "--- Cleaning up test environment ---"
    docker compose -f "$COMPOSE_FILE" down --remove-orphans
}

echo "--- Building and starting test environment ---"
# --abort-on-container-exit: Herhangi bir konteyner durduğunda (test-runner dahil) tüm süreci durdurur.
# --exit-code-from test-runner: 'docker compose' komutunun çıkış kodunu 'test-runner' servisinin
#                               çıkış kodu olarak ayarlar. Testler başarısız olursa, bu script de
#                               başarısız olur.
docker compose -f "$COMPOSE_FILE" up --build --abort-on-container-exit --exit-code-from test-runner

# 'trap' komutu sayesinde cleanup fonksiyonu burada otomatik olarak çalışacaktır.