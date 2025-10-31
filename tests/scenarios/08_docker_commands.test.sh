#!/bin/bash
# Bu test, test-runner konteynerinin (bir kullanıcı makinesini simüle ederek)
# Sentiric DNS proxy'si aktifken temel Docker komutlarını sorunsuzca
# çalıştırabildiğini doğrular.

set -e
set -x

# 'helpers.sh'i bu testte kullanmıyoruz çünkü proxy ayarlarını değil,
# şeffaf DNS'in etkisini test ediyoruz.
. "$(dirname "$0")/../helpers.sh"

# Testler bittiğinde ortamı temizlemek için bir fonksiyon
cleanup() {
  echo "--- Cleaning up Docker test artifacts ---"
  # -p ile projeye özel bir isim vererek orphan uyarısını engelle
  docker compose -p sentiric-dockertest -f compose.test.yml down --rmi all --volumes || true
  docker rmi docker-test-image:latest || true
  docker rmi hello-world:latest || true
  rm -f Dockerfile.test compose.test.yml
}
trap cleanup EXIT

print_header "Testing Docker Command Compatibility over DNS Proxy"

# --- Test 1: docker pull ---
# Docker Hub ile temel HTTPS iletişimini test eder.
echo "--- Step 1: Running 'docker pull hello-world' ---"
docker pull hello-world
echo "--- SUCCESS: 'docker pull' completed successfully. ---"

# --- Test 2: docker build ---
# Bir base imaj indirmeyi ve yerel build işlemini test eder.
echo "--- Step 2: Running 'docker build' ---"
# Anlık bir Dockerfile oluştur
echo -e 'FROM alpine:latest\nCMD ["echo", "Build successful!"]' > Dockerfile.test
# DÜZELTME: -f bayrağı ile doğru Dockerfile ismini belirt.
docker build -t docker-test-image -f Dockerfile.test .
echo "--- SUCCESS: 'docker build' completed successfully. ---"

# --- Test 3: docker compose up ---
echo "--- Step 3: Running 'docker compose up' ---"
echo -e 'services:\n  web:\n    image: nginx:alpine' > compose.test.yml
# -p ile projeye özel bir isim ver
docker compose -p sentiric-dockertest -f compose.test.yml up -d --quiet-pull

# Servisin ayağa kalktığını doğrula (DAHA GÜVENİLİR YÖNTEM)
SERVICE_STATE=$(docker compose -p sentiric-dockertest -f compose.test.yml ps --format '{{.State}}' web)
if [ "$SERVICE_STATE" != "running" ]; then
    echo "--- FAILURE: Docker compose service 'web' is not in 'running' state. State is: $SERVICE_STATE ---"
    exit 1
fi
echo "--- SUCCESS: 'docker compose up' completed successfully and service is running. ---"

# Cleanup fonksiyonu script sonunda otomatik olarak çalışacak.