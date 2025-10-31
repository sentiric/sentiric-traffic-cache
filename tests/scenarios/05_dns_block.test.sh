#!/bin/bash
set -e

. "$(dirname "$0")/../helpers.sh"

TEST_URL_BLOCK="https://www.google.com"

print_header "Testing Smart DNS End-to-End: BLOCK"

# Bu test, proxy ortam değişkenleri OLMADAN çalıştırılır.
# Trafiğin doğru şekilde engellenmesi, DNS tabanlı şeffaf proxy'nin
# çalıştığını kanıtlar. `curl`'un başarısız olması, testin başarılı olduğu anlamına gelir.
echo "--- RUNNING (direct): curl ${TEST_URL_BLOCK} ---"
if run_direct_curl ${TEST_URL_BLOCK} -o ${OUTPUT_FILE} --fail; then
    echo "--- FAILURE: Request to ${TEST_URL_BLOCK} was NOT blocked via DNS proxy. ---"
    exit 1
else
    echo "--- SUCCESS: Request was correctly blocked via DNS transparent proxy. ---"
fi