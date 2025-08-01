import os
from flask import Flask, jsonify, request
import mlflow

app = Flask(__name__)

model_uri = f"runs:/{os.environ['MODEL-ID']}/model"

MODEL = mlflow.tensorflow.load_model(model_uri)


@app.route("/predict", methods=["POST"])
def get_next():
    global MODEL

    response = {"is_a_dog": MODEL(request.json["content"])}

    return jsonify(response)


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=3000)
