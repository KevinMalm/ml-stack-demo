import os
import mlflow
import json


def main():
    mlflow_configure()

    with mlflow.start_run():
        pass


def mlflow_configure():
    # Load Env Vars
    _experiment = os.getenv("MLFLOW_EXPERIMENT_NAME")
    _tags = os.getenv("MLFLOW_EXPERIMENT_TAGS")
    # Validate
    if _experiment is None or _tags is None:
        raise Exception("Environment has not been setup correctly.")
    # Parse the ML Flow TAGs
    tags = json.loads(_tags.replace("\\", ""))

    # Set the Experiment name
    mlflow.set_experiment(_experiment)
    # Set the Experiment tags
    for x in tags:
        mlflow.set_tag(x["key"], x["value"])


if __name__ == "__main__":
    main()
