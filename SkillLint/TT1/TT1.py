import os

with open(os.getenv("LOG_PATH"), "w") as handle:
    handle.write("started")
