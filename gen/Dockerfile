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

# Copy all proto files
COPY . ./

# Create a mount point for the prost directory
VOLUME /app/prost

# Override default entrypoint
ENTRYPOINT []

# Use shell form of CMD
CMD ["/bin/sh", "-c", "\
    # Save our local proto files
    cp -rf dymensionxyz /tmp/dymensionxyz_local && \
    \
    # Clone hyperlane-cosmos for all proto dependencies
    git clone --branch main-dym --single-branch https://github.com/dymensionxyz/hyperlane-cosmos.git && \
    \
    # Copy all proto dependencies (includes IBC, cosmos, hyperlane, etc.)
    cp -rf hyperlane-cosmos/proto/* ./ && \
    rm -rf hyperlane-cosmos && \
    \
    # Overlay our local dymension proto files
    # This ensures we use our versions which may have updates
    cp -rf /tmp/dymensionxyz_local/* dymensionxyz/ && \
    rm -rf /tmp/dymensionxyz_local && \
    \
    # Generate protos
    buf generate --include-imports && \
    \
    # Clean up unnecessary generated files
    rm -rf prost/cosmos* prost/amino* prost/gogo* prost/google* \
"]