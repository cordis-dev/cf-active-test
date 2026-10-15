import os

log_path = os.environ["LOG_PATH"]
with open(log_path, "w") as handle:
    handle.write("started")
