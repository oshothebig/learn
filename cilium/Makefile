IMAGE := bird1

PHONY: deploy
deploy: cluster image
	kind load docker-image $(IMAGE)
	kubectl apply -f bird.yaml

PHONY: cluster
cluster:
	kind create cluster --config kind-config.yaml
	helm template cilium/cilium --version 1.12.3 --namespace kube-system --values values.yaml | kubectl apply -f -

PHONY: image
image:
	docker build -t $(IMAGE) .
