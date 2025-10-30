#!/bin/bash
set -e

CERT_PATH="/app/.certs/ca.crt"

# App konteynerinin sertifikayı oluşturmasını bekle
echo "Waiting for CA certificate to be generated..."
while [ ! -f "$CERT_PATH" ]; do
  sleep 1
done
echo "CA certificate found. Installing..."

# Sertifikayı sistemin güvenilir sertifikaları arasına kopyala ve güncelle
cp "$CERT_PATH" /usr/local/share/ca-certificates/sentiric-ca.crt
update-ca-certificates

echo "CA certificate installed."

# Dockerfile'daki asıl CMD'yi çalıştır
exec "$@"