FROM bufbuild/buf:latest as BUILDER
FROM node:20-alpine

COPY --from=BUILDER /usr/local/bin /usr/local/bin

# Inject user id and group id to avoid permission issues when running as a root user
ARG USER_ID=1000
ARG USER_GID=1000

# Create group if it doesn't exist
RUN if ! getent group $USER_GID; then \
    addgroup -g $USER_GID mygroup; \
    fi

# Create user if it doesn't exist
RUN if ! getent passwd $USER_ID; then \
    adduser -D -u $USER_ID -G $(getent group $USER_GID | cut -d: -f1) myuser; \
    fi

WORKDIR /app

RUN apk add --no-cache git openssh-client

COPY buf.gen.yaml ./

# Don't copy dymensionxyz here - we'll handle it in the CMD

# Create a mount point for the prost directory
VOLUME /app/prost

# Override default entrypoint
ENTRYPOINT []

# Use shell form of CMD
CMD ["/bin/sh", "-c", "\
    git clone --branch main-dym --single-branch https://github.com/dymensionxyz/hyperlane-cosmos.git && \
    cp -rf hyperlane-cosmos/proto/* ./ && \
    rm -rf hyperlane-cosmos && \
\
    # pull cosmos/ics23 dependency for IBC protos
    git clone --depth 1 https://github.com/cosmos/ics23.git && \
    mkdir -p cosmos/ics23/v1 && \
    cp -rf ics23/proto/cosmos/ics23/v1/* ./cosmos/ics23/v1/ && \
    rm -rf ics23 && \
\
    # pull x/kas protos from dymension
    git clone https://github.com/dymensionxyz/dymension.git && \
    mkdir -p dymensionxyz/dymension && \
    cp -rf dymension/proto/dymensionxyz/dymension/kas ./dymensionxyz/dymension/ && \
    rm -rf dymension && \
\
    # create forward directory for mounted dt.proto
    mkdir -p dymensionxyz/dymension/forward && \
\
    buf generate --include-imports && \
    rm -rf prost/cosmos* prost/amino* prost/gogo* prost/google* \
"]