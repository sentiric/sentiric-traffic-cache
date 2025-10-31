#!/bin/bash

export APP_HOST="app"
export PROXY_URL="http://${APP_HOST}:3128"
export MGMT_URL="http://${APP_HOST}:8080"
export OUTPUT_FILE="/dev/null"
export CA_CERT_PATH="/app/.certs/ca.crt"

# Fonksiyon: Mesajı başlık formatında yazdır
function print_header {
    echo ""
    echo "================================================="
    echo ">> $1"
    echo "================================================="
}

# Fonksiyon: Bir test adımını raporlar
function print_step {
    echo "--- STEP: $1 ---"
}

# Proxy üzerinden güvenli curl isteği yapan fonksiyon
function run_proxied_curl {
    curl -s -L --proxy ${PROXY_URL} --cacert ${CA_CERT_PATH} "$@"
}

# Proxy OLMADAN, sadece DNS yönlendirmesiyle güvenli curl isteği yapar
function run_direct_curl {
    curl -s -L --cacert ${CA_CERT_PATH} "$@"
}

# Bir komutun BAŞARILI olmasını bekler ve sonucu raporlar.
function assert_success {
    local message=$1
    shift
    if "$@"; then
        echo "✅ SUCCESS: $message"
    else
        echo "❌ FAILURE: $message"
        exit 1
    fi
}

# Bir komutun BAŞARISIZ olmasını bekler ve sonucu raporlar.
function assert_failure {
    local message=$1
    shift
    if ! "$@"; then
        echo "✅ SUCCESS: $message"
    else
        echo "❌ FAILURE: $message"
        exit 1
    fi
}

function capture_initial_stats {
    print_step "Capturing initial stats (clearing cache first)"
    curl -s -X POST ${MGMT_URL}/api/clear > ${OUTPUT_FILE}
    sleep 1
    
    local stats=$(curl -s ${MGMT_URL}/api/stats)
    
    if ! echo "$stats" | jq empty; then
        echo "❌ FAILURE: Failed to get valid JSON stats from API. Response was: $stats"
        exit 1
    fi
    
    INITIAL_HITS=$(echo ${stats} | jq '.hits')
    INITIAL_MISSES=$(echo ${stats} | jq '.misses')
}

function assert_stats_increment {
    local expected_hits_increment=$1
    local expected_misses_increment=$2
    local context=$3
    
    sleep 1
    local stats=$(curl -s ${MGMT_URL}/api/stats)

    if ! echo "$stats" | jq empty; then
        echo "❌ FAILURE ($context): Failed to get valid JSON stats. Response: $stats"
        exit 1
    fi
    
    local actual_hits=$(echo ${stats} | jq '.hits')
    local actual_misses=$(echo ${stats} | jq '.misses')
    local expected_total_hits=$((INITIAL_HITS + expected_hits_increment))
    local expected_total_misses=$((INITIAL_MISSES + expected_misses_increment))

    if [ "${actual_hits}" -ne "${expected_total_hits}" ]; then
        echo "❌ FAILURE ($context): Expected total hits=${expected_total_hits}, but got ${actual_hits}"
        exit 1
    fi
    
    if [ "${actual_misses}" -ne "${expected_total_misses}" ]; then
        echo "❌ FAILURE ($context): Expected total misses=${expected_total_misses}, but got ${actual_misses}"
        exit 1
    fi
    
    echo "✅ SUCCESS ($context): Stats incremented correctly (Total Hits: ${actual_hits}, Total Misses: ${actual_misses})"
}