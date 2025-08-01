mlflow server \
  --backend-store-uri $BACKEND_STORE_URI \
  --default-artifact-root $ARTIFACT_ROOT \
  --host 0.0.0.0 \
  --port $ML_FLOW_PORT
