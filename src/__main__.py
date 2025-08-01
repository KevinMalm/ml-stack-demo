import os
from flask import Flask, jsonify
import random
import string
from breeds import BREEDS

app = Flask(__name__)


def generate_record():
    is_dog = random.random() < 0.4
    my_str = (
        random.choice(BREEDS)
        if is_dog
        else "".join(
            random.choices(
                string.ascii_letters + string.digits, k=random.randint(4, 34)
            )
        )
    )

    ascii_arr = [ord(char) for char in my_str]

    return ascii_arr, my_str, is_dog


@app.route("/live", methods=["GET"])
def get_live():
    ascii_string, _truth_string, _is_dog = generate_record()
    response = {"content": ascii_string}

    return jsonify(response)


@app.route("/test", methods=["GET"])
def get_test():
    ascii_string, truth_string, is_dog = generate_record()
    response = {"content": ascii_string, "value": truth_string, "flag": is_dog}

    return jsonify(response)


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=3000)
