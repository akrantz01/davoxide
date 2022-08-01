FROM node:16 as frontend

WORKDIR frontend

# Install dependencies
COPY frontend/package.json .
COPY frontend/yarn.lock .
RUN yarn install

# Add project files
COPY frontend/index.html .
COPY frontend/public ./public
COPY frontend/src ./src
COPY frontend/tsconfig.json .
COPY frontend/tsconfig.node.json .
COPY frontend/vite.config.ts .

# Build project
RUN yarn build


FROM rust:1.62 as builder

RUN cargo new --bin davoxide
WORKDIR davoxide

# Install dependencies
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

# Add project files
COPY --from=frontend /frontend/dist ./frontend/dist
COPY build.rs .
COPY sqlx-data.json .
COPY migrations ./migrations
COPY src ./src

# Compile project
RUN rm ./target/release/davoxide*
RUN cargo build --release


FROM debian:11-slim

RUN apt-get update && \
    apt-get install -y ca-certificates tzdata && \
    rm -rf /var/lib/apt/lists/*

# Switch to non-root user
RUN adduser --disabled-password app
USER app

# Configure defaults
ENV ADDRESS 0.0.0.0:3000
ENV RUST_LOG info
ENV TZ Etc/UTC

EXPOSE 3000

COPY --from=builder --chown=app /davoxide/target/release/davoxide /davoxide

CMD ["./davoxide"]
