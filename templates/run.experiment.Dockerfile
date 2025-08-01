# syntax=docker/dockerfile:1

# BASE PACKAGE
FROM python:3.12-slim

# ARGUMENTS
ARG MODEL_REPO
ARG MODEL_NAME
ARG MODEL_TAGS
ARG MLFLOW_HOST
ARG MLFLOW_PORT
ARG GITHUB_LIB_BRANCH
ARG GITHUB_LIB_REPO
ARG SERVER_URL

# Expose the Port
EXPOSE $MLFLOW_PORT
EXPOSE 3000

# INSTALL DEPENDENCIES
RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# SET CONTEXT VARIABLES
ENV VIRTUAL_ENV=/opt/venv
ENV PATH="$VIRTUAL_ENV/bin:$PATH"

# ACTIVATE PYTHON ENV
RUN python -m venv $VIRTUAL_ENV

# MAKE ROOT DIRECTORY
RUN mkdir -p /app

# SET WORKING DIRECTORY
WORKDIR /app

# INSTALL THE ML-STACK HELPER LIBRARY
RUN pip install --upgrade pip setuptools wheel build

RUN git clone --branch ${GITHUB_LIB_BRANCH} ${GITHUB_LIB_REPO} /tmp/github-lib && \
    cd /tmp/github-lib && \
    python -m build && \
    pip install dist/*.whl


# PULL MODEL REPO
RUN git clone --branch ${MODEL_NAME} ${MODEL_REPO} /app
# INSTALL ALL REQUIREMENTS
RUN pip install -r requirements.txt

# SET ENVIRONMENT VARIABLES
ENV MLFLOW_EXPERIMENT_TAGS=${MODEL_TAGS}
ENV SERVER_URL=${SERVER_URL}
ENV MLFLOW_URL="http://${MLFLOW_HOST}:${MLFLOW_PORT}"

# Set MLFLOW_EXPERIMENT_NAME using MODEL_NAME and git short hash
RUN GIT_HASH=$(git -C /app rev-parse --short HEAD) && \
    echo "export MLFLOW_EXPERIMENT_NAME=${MODEL_NAME}-$GIT_HASH" >> /etc/profile.d/mlflow_env.sh

CMD [ "/bin/sh", "-c", ". /etc/profile.d/mlflow_env.sh && python src/__main__.py"]
