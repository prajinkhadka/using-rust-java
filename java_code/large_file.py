import json
# twitter.json -> https://github.com/serde-rs/json-benchmark/blob/master/data/twitter.json
with open("twitter.json") as f:
    original = json.load(f)

repeated = [original] * 1000

with open("twitter_large.json", "w") as f:
    json.dump(repeated, f)

