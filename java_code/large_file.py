import json

with open("twitter.json") as f:
    original = json.load(f)

repeated = [original] * 1000

with open("twitter_large.json", "w") as f:
    json.dump(repeated, f)

