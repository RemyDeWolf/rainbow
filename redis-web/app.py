import time

import redis
from flask import Flask

app = Flask(__name__)
cache = redis.Redis(host='redis', port=6379, decode_responses=True)

def get_computed_count():
    retries = 5
    while True:
        try:
            result = ""
            for key in cache.keys("*.computed"):
                result += '{}: {}<br>\n'.format(key, cache.get(key))
            return result
        except redis.exceptions.ConnectionError as exc:
            if retries == 0:
                raise exc
            retries -= 1
            time.sleep(0.5)

@app.route('/')
def main():
    return get_computed_count()
