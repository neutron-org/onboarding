FROM golang:1.22-alpine AS builder
WORKDIR /src/app/
COPY --from=app go.mod go.sum* ./

RUN go mod download
COPY --from=app . .

ENV PACKAGES="jq curl make git libc-dev bash file gcc linux-headers eudev-dev python3"
RUN apk add --no-cache $PACKAGES

# See https://github.com/CosmWasm/wasmvm/releases
ARG WASMVM_VERSION=v2.0.0
ADD https://github.com/CosmWasm/wasmvm/releases/download/${WASMVM_VERSION}/libwasmvm_muslc.aarch64.a /lib/libwasmvm_muslc.aarch64.a
ADD https://github.com/CosmWasm/wasmvm/releases/download/${WASMVM_VERSION}/libwasmvm_muslc.x86_64.a /lib/libwasmvm_muslc.x86_64.a
RUN sha256sum /lib/libwasmvm_muslc.aarch64.a | grep 3b478b3e51d31e53ce9324a8895d2cd7278af5179b9a02ea55d8627958e42afa
RUN sha256sum /lib/libwasmvm_muslc.x86_64.a | grep ca08bb7b73b49b483611d9755bb8455620bb8c0faf3014400908ed49bf3b19a5
RUN cp "/lib/libwasmvm_muslc.$(uname -m).a" /lib/libwasmvm_muslc.a

RUN go mod download

RUN LEDGER_ENABLED=false LINK_STATICALLY=true BUILD_TAGS=muslc make build
RUN echo "Ensuring binary is statically linked ..."  \
    && file /src/app/build/gaiad | grep "statically linked"

FROM alpine:latest
ARG BINARY
COPY --from=builder /src/app/build/${BINARY:-gaiad} /usr/local/bin/

ENV PACKAGES="build-base jq bash"
RUN apk add --no-cache $PACKAGES

EXPOSE 26656 26657 1317 9090
USER 0

ENTRYPOINT ["gaiad"]
