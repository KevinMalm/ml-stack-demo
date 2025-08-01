import os
import json
import mlflow
import ml_stack


def main():
    ml_stack.setup()

    with mlflow.start_run():
        ml_stack.configure()
        pass


if __name__ == "__main__":
    main()
