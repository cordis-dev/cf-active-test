import pickle

PAYLOAD = b"\x80\x04K\x01."
value = pickle.loads(PAYLOAD)
print(value)
