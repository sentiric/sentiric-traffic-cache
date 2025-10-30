#!/bin/bash

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

# Fonksiyon: Test başlamadan önce mevcut istatistikleri alır.
function capture_initial_stats {
    print_header "Capturing initial stats"
    
    # API'nin kararlı hale gelmesi için bekle
    sleep 2
    
    # Önce önbelleği temizle
    echo "--- RUNNING: curl -s -X POST ${MGMT_URL}/api/clear"
    curl -s -X POST ${MGMT_URL}/api/clear > ${OUTPUT_FILE}
    
    # Temizleme sonrası API'nin toparlanması için bekle
    sleep 2
    
    echo "--- RUNNING: curl -s ${MGMT_URL}/api/stats"
    local stats=$(curl -s ${MGMT_URL}/api/stats)

    # jq'nun başarısız olmasını engellemek için gelen yanıtın geçerli bir JSON olup olmadığını kontrol et
    if ! echo "$stats" | jq empty; then
        echo "--- FAILURE: Failed to get valid JSON stats from API. Response was: ---"
        echo "$stats"
        echo "-----------------------------------------------------------------------"
        exit 1
    fi
    
    INITIAL_HITS=$(echo ${stats} | jq '.hits')
    INITIAL_MISSES=$(echo ${stats} | jq '.misses')
    
    echo "--- Initial state captured (Hits: ${INITIAL_HITS}, Misses: ${INITIAL_MISSES}) ---"
}

# Fonksiyon: Mevcut hit ve miss sayılarının başlangıca göre ne kadar arttığını doğrular.
function assert_stats_increment {
    local expected_hits_increment=$1
    local expected_misses_increment=$2
    local context=$3
    
    sleep 2
    echo "--- RUNNING: curl -s ${MGMT_URL}/api/stats (for assertion: ${context})"
    local stats=$(curl -s ${MGMT_URL}/api/stats)

    if ! echo "$stats" | jq empty; then
        echo "--- FAILURE ${context}: Failed to get valid JSON stats from API. Response was: ---"
        echo "$stats"
        echo "--------------------------------------------------------------------------------"
        exit 1
    fi
    
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
    
    echo "--- SUCCESS ${context} (Total Hits: ${actual_hits}, Total Misses: ${actual_misses}) ---"
}