#!/bin/bash

export APP_HOST="app"
export PROXY_URL="http://${APP_HOST}:3128"
export MGMT_URL="http://${APP_HOST}:8080"
export OUTPUT_FILE="/dev/null"

function print_header {
    echo ""
    echo "================================================="
    echo ">> $1"
    echo "================================================="
    # Komutların loglarda daha net görünmesi için
    set -x
}

function run_command {
    # Komutların loglarda daha net görünmesi için
    set +x
    echo "--- RUNNING: $*"
    set -x
    "$@"
    set +x
}

# Fonksiyon: Test başlamadan önce mevcut istatistikleri alır.
function capture_initial_stats {
    print_header "Capturing initial stats"
    
    # İstatistiklerin oturması için bekle
    sleep 2
    
    # Önce önbelleği temizleyerek başlayalım, ancak sonucunu kontrol etmeyelim.
    # Bu sadece bir sonraki testin "temiz" bir diskle başlamasını sağlar.
    run_command curl -s -X POST ${MGMT_URL}/api/clear -o ${OUTPUT_FILE}
    
    # API'nin kendini toparlaması ve olası artçı isteklerin bitmesi için bekle
    sleep 2
    
    local stats=$(run_command curl -s ${MGMT_URL}/api/stats)
    INITIAL_HITS=$(echo ${stats} | jq '.hits')
    INITIAL_MISSES=$(echo ${stats} | jq '.misses')
    
    set +x
    echo "--- Initial state captured (Hits: ${INITIAL_HITS}, Misses: ${INITIAL_MISSES}) ---"
}

# Fonksiyon: Mevcut hit ve miss sayılarının başlangıca göre ne kadar arttığını doğrular.
function assert_stats_increment {
    local expected_hits_increment=$1
    local expected_misses_increment=$2
    local context=$3
    
    # İstatistiklerin güncellenmesi için yeterli zaman ver
    sleep 2
    local stats=$(run_command curl -s ${MGMT_URL}/api/stats)
    local actual_hits=$(echo ${stats} | jq '.hits')
    local actual_misses=$(echo ${stats} | jq '.misses')

    local expected_total_hits=$((INITIAL_HITS + expected_hits_increment))
    local expected_total_misses=$((INITIAL_MISSES + expected_misses_increment))

    if [ "${actual_hits}" -ne "${expected_total_hits}" ]; then
        echo "--- FAILURE ${context}: Expected total hits to be ${expected_total_hits}, but got ${actual_hits} ---"
        exit 1
    fi
    
    if [ "${actual_misses}" -ne "${expected_total_misses}" ]; then
        echo "--- FAILURE ${context}: Expected total misses to be ${expected_total_misses}, but got ${actual_misses} ---"
        exit 1
    fi
    
    set +x
    echo "--- SUCCESS ${context} (Total Hits: ${actual_hits}, Total Misses: ${actual_misses}) ---"
}