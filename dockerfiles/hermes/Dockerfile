FROM ubuntu:24.04
ARG HERMES_VERSION=v1.10.4
WORKDIR /app
RUN apt-get update && apt-get install -y wget && \
    rm -rf /var/lib/apt/lists/* && \
    apt-get clean && \
    PLATFORM=`uname -a | awk '{print $(NF-1)}'` && \
    VERSION=${HERMES_VERSION} && \
    TARNAME="hermes-${VERSION}-${PLATFORM}-unknown-linux-gnu.tar.gz" && \
    wget "https://github.com/informalsystems/hermes/releases/download/${VERSION}/${TARNAME}" && \
    tar -xf "$TARNAME" && \
    mv ./hermes /bin/hermes && \
    rm -rf "$TARNAME"

ENTRYPOINT ["hermes", "start"]
