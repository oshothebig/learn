export DOCKER_BUILDKIT := 1

BGP_PEERING ?= numbered
FRR_CONFIG_FILES := $(wildcard config/$(BGP_PEERING)/*.conf)
TINET_CONFIG_FILE := $(BGP_PEERING).yaml

.PHONY: prepare
prepare: image tinet

.PHONY: image
image:
	docker build -t frr-bgp -f frr.Dockerfile .

tinet:
	docker build -o . - < tinet.Dockerfile

.PHONY: network
network:
	./tinet up -c $(TINET_CONFIG_FILE) | sudo sh -x
	./tinet conf -c $(TINET_CONFIG_FILE) | sudo sh -x

.PHONY: rebuild
rebuild:
	./tinet reconf -c $(TINET_CONFIG_FILE) | sudo sh -x

.PHONY: destroy
destroy:
	./tinet down -c $(TINET_CONFIG_FILE) | sudo sh -x

.PHONY: load-config
load-config: $(FRR_CONFIG_FILES)

.PHONY: $(FRR_CONFIG_FILES)
$(FRR_CONFIG_FILES):
	docker cp $@ $(basename $(notdir $@)):/etc/frr/frr.conf
	docker exec $(basename $(notdir $@)) /usr/lib/frr/frr-reload.py --reload /etc/frr/frr.conf
