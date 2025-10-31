#!/bin/bash
set -e

CERT_PATH="/app/.certs/ca.crt"

echo "Waiting for CA certificate to be generated..."
# Sertifikanın oluşması için bekleme döngüsü
ATTEMPTS=0
MAX_ATTEMPTS=30
while [ ! -f "$CERT_PATH" ]; do
  if [ $ATTEMPTS -ge $MAX_ATTEMPTS ]; then
    echo "ERROR: Certificate file not found after $MAX_ATTEMPTS seconds."
    exit 1
  fi
  sleep 1
  ATTEMPTS=$((ATTEMPTS + 1))
done
echo "CA certificate found. Installing..."

# Sertifikayı sistemin güvenilir sertifikaları arasına kopyala ve güncelle
cp "$CERT_PATH" /usr/local/share/ca-certificates/sentiric-ca.crt
# || true ekleyerek, komut sıfırdan farklı bir kodla çıksa bile script'in devam etmesini sağla
update-ca-certificates || true

echo "CA certificate installed. Handing over to main command..."

# Dockerfile'daki asıl CMD'yi veya docker-compose'daki 'command'ı çalıştır
exec "$@"