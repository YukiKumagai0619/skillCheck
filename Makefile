YML_PATH=./docker/docker-compose.yml
CONF_PATH=./conf/sysctl.conf

build:
	docker-compose -f $(YML_PATH) build

run:
	docker-compose -f $(YML_PATH) run --rm rust /bin/bash -c "cargo run $(CONF_PATH)"
