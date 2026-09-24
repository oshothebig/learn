FROM ubuntu:24.04

ARG DEBIAN_FRONTEND=noninteractive
# "stable" or "nightly"
ARG ZEBRA_RS_CHANNEL=stable

# libcap2-bin: the zebra-rs postinst runs setcap
# iproute2 etc.: tools for inspecting routes and building lab networks
RUN apt update \
    && apt install -y --no-install-recommends \
        ca-certificates \
        curl \
        iproute2 \
        iputils-ping \
        kmod \
        less \
        libcap2-bin \
        procps \
        tcpdump \
        traceroute \
    && rm -rf /var/lib/apt/lists/*

# Register the zebra-rs APT repository (same as https://zebra.rs/install.sh)
RUN set -eu; \
    case "${ZEBRA_RS_CHANNEL}" in \
        stable) tag=apt ;; \
        nightly) tag=nightly ;; \
        *) echo "invalid ZEBRA_RS_CHANNEL: ${ZEBRA_RS_CHANNEL}" >&2; exit 1 ;; \
    esac; \
    . /etc/os-release; \
    install -d -m 0755 /etc/apt/keyrings; \
    curl -fsSL https://zebra.rs/apt/zebra-rs-archive-keyring.asc -o /etc/apt/keyrings/zebra-rs.asc; \
    printf 'Types: deb\nURIs: %s\nSuites: ./\nSigned-By: %s\n' \
        "https://github.com/zebra-rs/zebra-rs.github.io/releases/download/${tag}-${VERSION_CODENAME}" \
        /etc/apt/keyrings/zebra-rs.asc \
        > /etc/apt/sources.list.d/zebra-rs.sources

# cradle-rs (eBPF data plane) is only a Recommends, so skip it
RUN apt update \
    && apt install -y --no-install-recommends zebra-rs \
    && rm -rf /var/lib/apt/lists/*

CMD ["/usr/bin/zebra-rs"]
