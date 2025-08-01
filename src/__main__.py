import os
import json
import mlflow
import ml_stack


def main():
    ml_stack.configure()

    with mlflow.start_run():
        pass


if __name__ == "__main__":
    main()
