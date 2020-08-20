import os
import time
from random import seed
from random import randint

import uuid

# seed random number generator
seed(time.time())

def compute():

    size = int(os.environ['HASHTABLE_SIZE'])
    read_count = int(os.environ['READ_COUNT'])

    # create an input map and an array
    hashtable = {}
    keys = [0] * size

    # init the map
    for i in range(size):
        key = uuid.uuid1()
        hashtable[key]=key
        keys[i]=key

    #access map
    for i in range(read_count):
        key = keys[randint(0, size-1)]
        _ = hashtable[key]
