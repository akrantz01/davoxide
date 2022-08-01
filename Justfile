default:
    @just --list

# Start the API in debug
dev-api:
    cargo run

# Start the UI dev server
dev-ui:
    cd frontend && yarn dev

# Create a production build
release: ui-release api-release

# Create a production build of the API
api-release:
    cargo build --release

# Create a production build of the UI
ui-release:
    cd frontend && yarn build

clean:
    rm -rf frontend/dist

test:
    cargo test
