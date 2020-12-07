from flask import Flask, request
from flask_cors import CORS
import time
import json

app = Flask("server")
cors = CORS(app)

@app.route("/")
def hw():
    print("Hello world")

@app.route("/postResponse", methods=["POST"])
def postResponse():
    req = request.form.to_dict()
    print(req)
    return json.dumps({"correct": True, "timeOfFinish": time.time() * 1000})

app.run("0.0.0.0",port=80)