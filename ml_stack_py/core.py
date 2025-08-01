import os
import mlflow
import json


def setup():
    # Load Env Vars
    url = os.getenv("MLFLOW_URL", "http://127.0.0.1:5050")
    # Parse the ML Flow TAGs
    print(f"Configuring the ML Flow host to {url}")
    mlflow.set_tracking_uri(url)


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
