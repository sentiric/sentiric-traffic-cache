#!/bin/bash

# Bu script doğrudan çalıştırılmak için değil, diğer script'ler tarafından kaynak olarak kullanılmak içindir.
# Bu yüzden 'set -e' ve 'set -x' burada yok.

# Ortam değişkenleri ve genel ayarlar
export APP_HOST="app"
export PROXY_URL="http://${APP_HOST}:3128"
export MGMT_URL="http://${APP_HOST}:8080"
export OUTPUT_FILE="/dev/null"

# Fonksiyon: Mesajı başlık formatında yazdır
function print_header {
    echo ""
    echo "================================================="
    echo ">> $1"
    echo "================================================="
}

# Fonksiyon: İstatistikleri temizler ve sıfır olduğunu doğrular
function clear_stats {
    print_header "Clearing cache and stats"
    curl -s -X POST ${MGMT_URL}/api/clear > ${OUTPUT_FILE}
    # API'nin kendini toparlaması için kısa bir bekleme
    sleep 1
    local stats=$(curl -s ${MGMT_URL}/api/stats)
    local hits=$(echo ${stats} | jq '.hits')
    local misses=$(echo ${stats} | jq '.misses')

    if [ "${hits}" -ne 0 ] || [ "${misses}" -ne 0 ]; then
        echo "--- FAILURE: Stats are not zero after clear! Hits: ${hits}, Misses: ${misses} ---"
        exit 1
    fi
    echo "--- Stats cleared successfully ---"
}

# Fonksiyon: Belirtilen hit ve miss sayılarını doğrular
function assert_stats {
    local expected_hits=$1
    local expected_misses=$2
    local context=$3
    
    sleep 1 # Verinin güncellenmesi için bekle
    local stats=$(curl -s ${MGMT_URL}/api/stats)
    local actual_hits=$(echo ${stats} | jq '.hits')
    local actual_misses=$(echo ${stats} | jq '.misses')

    if [ "${actual_hits}" -ne "${expected_hits}" ]; then
        echo "--- FAILURE ${context}: Expected ${expected_hits} hits, got ${actual_hits} ---"
        exit 1
    fi
    
    if [ "${actual_misses}" -ne "${expected_misses}" ]; then
        echo "--- FAILURE ${context}: Expected ${expected_misses} misses, got ${actual_misses} ---"
        exit 1
    fi
    
    echo "--- SUCCESS ${context} (Hits: ${actual_hits}, Misses: ${actual_misses}) ---"
}