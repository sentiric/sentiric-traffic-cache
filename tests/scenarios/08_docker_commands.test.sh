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
  # DÜZELTME: Dosya varsa komutu çalıştır, yoksa hata verme.
  if [ -f "compose.test.yml" ]; then
    docker compose -f compose.test.yml down --rmi all --volumes
  fi
  docker rmi docker-test-image || true
  docker rmi hello-world alpine nginx || true
  # DÜZELTME: Var olan dosyaları güvenli bir şekilde sil.
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
# Birden fazla imaj çekme ve ağ oluşturma yeteneğini test eder.
echo "--- Step 3: Running 'docker compose up' ---"
# Anlık bir compose dosyası oluştur
echo -e 'services:\n  web:\n    image: nginx:alpine' > compose.test.yml
# -d: arkaplanda çalıştır, --quiet-pull: pull loglarını kısalt
docker compose -f compose.test.yml up -d --quiet-pull
# Servisin ayağa kalktığını doğrula
docker compose -f compose.test.yml ps | grep "running"
echo "--- SUCCESS: 'docker compose up' completed successfully. ---"

# Cleanup fonksiyonu script sonunda otomatik olarak çalışacak.