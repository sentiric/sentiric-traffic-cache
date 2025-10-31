#!/bin/bash
set -e

. "$(dirname "$0")/../helpers.sh"

# Testler bittiğinde ortamı temizlemek için bir fonksiyon
cleanup() {
  echo "--- Cleaning up Docker test artifacts ---"
  docker compose -p sentiric-dockertest -f compose.test.yml down --rmi all --volumes &> /dev/null || true
  docker rmi docker-test-image:latest &> /dev/null || true
  docker rmi hello-world:latest &> /dev/null || true
  rm -f Dockerfile.test compose.test.yml
}
trap cleanup EXIT

print_header "SCENARIO 08: Docker Command Compatibility"

print_step "Running 'docker pull hello-world'"
assert_success "'docker pull' completed" docker pull hello-world

print_step "Running 'docker build'"
echo -e 'FROM alpine:latest\nCMD ["echo", "Build successful!"]' > Dockerfile.test
assert_success "'docker build' completed" docker build -t docker-test-image -f Dockerfile.test .

print_step "Running 'docker compose up'"
echo -e 'services:\n  web:\n    image: nginx:alpine' > compose.test.yml
assert_success "'docker compose up' completed" docker compose -p sentiric-dockertest -f compose.test.yml up -d --quiet-pull

print_step "Verifying docker compose service is running"
assert_success "Service is in 'running' state" \
    test "$(docker compose -p sentiric-dockertest -f compose.test.yml ps --format '{{.State}}' web)" = "running"