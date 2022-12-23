# dnsled

## setup
```bash
DNS_BIND=10.0.0.10:53 # listen port
DNS_UPSTREAM=9.9.9.10 # upstream DNS server
WLED_API=http://10.0.0.11/json/state # WLED API URL
LEDS=100 # number of leds
```

## run

```bash
docker-compose up -d
```