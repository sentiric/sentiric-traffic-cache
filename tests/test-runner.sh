#!/bin/bash
set -e # Herhangi bir komut hata verirse script'i durdur
set -x # Çalışan komutları ekrana yaz

echo "--- Waiting for proxy to be ready ---"
# Basit bir bekleme mekanizması
sleep 5

TEST_URL="http://cachefly.cachefly.net/10mb.test"
OUTPUT_FILE="/dev/null"

echo "--- Running Cache MISS test ---"
# İlk istek, cache boş olmalı
time curl -s -L $TEST_URL -o $OUTPUT_FILE

echo "--- Running Cache HIT test ---"
# İkinci istek, cache'den gelmeli ve çok daha hızlı olmalı
# `time` komutunun çıktısını bir dosyaya yazıp karşılaştıracağız
time (curl -s -L $TEST_URL -o $OUTPUT_FILE) 2> time_output.txt

# Cache hit süresinin 1 saniyeden az olduğunu varsayıyoruz.
# Bu basit bir kontrol, daha sonra geliştirilebilir.
if grep -q "real\s*0m0" time_output.txt; then
    echo "--- SUCCESS: Cache HIT was fast as expected! ---"
else
    echo "--- FAILURE: Cache HIT was too slow! ---"
    cat time_output.txt
    exit 1
fi

echo "--- ALL INTEGRATION TESTS PASSED ---"