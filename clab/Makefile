.PHONY: deploy
deploy:
	kind create cluster --config kind-config.yaml
	sudo clab deploy

.PHONY: destroy
destroy:
	sudo clab destroy
	kind delete cluster

cilium:
	./install-cilium.sh

.PHONY: install-cilium
install-cilium: cilium
	./cilium install --helm-values values.yaml
