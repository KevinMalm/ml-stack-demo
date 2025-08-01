mkdir -p "./dist"

source .ml-stack-demo/bin/activate

export "BACKEND_STORE_URI=$BACKEND_STORE_URI"
export "ARTIFACT_ROOT=$BACKEND_STORE_URI"
export "ML_FLOW_HOST=$ML_FLOW_HOST"
export "ML_FLOW_PORT=$ML_FLOW_PORT"

export RUST_LOG=debug
export CARGO_TARGET_DIR="$(pwd)/dist"
export PATH="$(pwd)/dist:$(pwd)/dist/debug:$PATH"

# Start MLflow server on port 5050
