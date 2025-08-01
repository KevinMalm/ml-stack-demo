import os
import mlflow
import json


def setup():
    # Load Env Vars
    _host = os.getenv("MLFLOW_HOST", "127.0.0.1")
    _port = os.getenv("MLFLOW_PORT", "5050")

    # Parse the ML Flow TAGs
    print("Configuring the ML Flow host")
    mlflow.set_tracking_uri(f"http://{_host}:{_port}")


def configure():
    # Load Env Vars
    _experiment = os.getenv("MLFLOW_EXPERIMENT_NAME")
    _tags = os.getenv("MLFLOW_EXPERIMENT_TAGS")
    # Validate
    if _experiment is None or _tags is None:
        print("WARNING: Environment has not been setup correctly.")
        return
    # Parse the ML Flow TAGs
    tags = json.loads(_tags.replace("\\", ""))

    # Set the Experiment name
    mlflow.set_experiment(_experiment)
    # Set the Experiment tags
    for x in tags:
        mlflow.set_tag(x["key"], x["value"])
