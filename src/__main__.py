import os
import json
import requests
import mlflow
import ml_stack_py
import numpy as np
import tensorflow as tf
from dataclasses import dataclass
from typing import List
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import Embedding, GlobalAveragePooling1D, Dense
from tensorflow.keras.preprocessing.sequence import pad_sequences
from sklearn.model_selection import train_test_split


_MAX_LEN = 34
_VOCAB_SIZE = 128
_EMBEDDING = 16


@dataclass
class ApiRecord:
    content: List[int]
    flag: bool
    value: str


def main():
    ml_stack_py.setup()
    with mlflow.start_run(run_name="Am-I-a-Dog?"):
        ml_stack_py.configure()
        mlflow.tensorflow.autolog()

        (training_x, training_y) = generate_training_data()
        (train_x, val_x), (train_y, val_y) = prep_data(training_x, training_y)
        model = build_model()
        history = model.fit(
            train_x,
            np.array(train_y),
            validation_data=(val_x, np.array(val_y)),
            epochs=50,
            verbose=1,
        )
        # Log into ML-Flow
        mlflow.log_param("embedding_dim", _EMBEDDING)
        mlflow.log_param("max_len", _MAX_LEN)
        val_loss, val_acc = model.evaluate(val_x, np.array(val_y), verbose=0)
        mlflow.log_metric("val_accuracy", val_acc)
        mlflow.log_metric("val_loss", val_loss)

        print(f"✅ Training complete. Validation Accuracy: {val_acc:.3f}")


def generate_training_data(n=100):
    host = os.getenv("SERVER_URL", "127.0.0.1")
    URL = f"http://{host}:3000/test"
    arr: List[ApiRecord] = []
    for _ in range(n):

        try:
            response = requests.get(URL)
            response.raise_for_status()
            data = response.json()

            arr.append(
                ApiRecord(
                    content=data.get("content", []),
                    flag=data.get("flag"),
                    value=data.get("value"),
                )
            )

        except Exception as e:
            print(f"An error occurred: {e}")
            raise e
    # Save file and log to ML-Flow
    _TRAINING_FILE = "training.set.csv"
    with open(_TRAINING_FILE, "w") as f:
        f.write("CONTENT,VALUE,FLAG\n")
        for x in arr:
            f.write(
                ",".join(
                    [
                        "|".join([repr(z) for z in x.content]),
                        x.value,
                        "1" if x.flag else "0",
                    ]
                )
                + "\n"
            )
    mlflow.log_artifact(_TRAINING_FILE)
    # Split X / Y
    return [x.content for x in arr], [1 if x.flag else 0 for x in arr]


def prep_data(x, y):
    x_padded = pad_sequences(x, maxlen=_MAX_LEN, padding="post")

    x_train, x_val, y_train, y_val = train_test_split(
        x_padded,
        y,
        test_size=0.4,
        stratify=y, # Keep things balanced!
    )
    return (x_train, x_val), (y_train, y_val)


def build_model():
    model = Sequential(
        [
            Embedding(
                input_dim=_VOCAB_SIZE, output_dim=_EMBEDDING, input_length=_MAX_LEN
            ),
            GlobalAveragePooling1D(),
            Dense(8, activation="relu"),
            Dense(1, activation="sigmoid"),
        ]
    )

    model.compile(optimizer="adam", loss="binary_crossentropy", metrics=["accuracy"])
    return model


if __name__ == "__main__":
    main()
