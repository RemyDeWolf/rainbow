import os
import time
from random import seed
from random import randint

# seed random number generator
seed(time.time())

def compute():

    writes = int(os.environ['WRITES'])
    reads = int(os.environ['READS'])
    file_size = int(os.environ['FILE_SIZE'])

    # create an array with the file names
    files = [""] * writes

    # create the file data
    data = "0" * file_size

    # write the files
    for i in range(writes):
        path = "/data/{}".format(i)
        f = open(path, "w")
        f.write(data)
        f.close()

    # read the files
    for i in range(reads):
        path = "/data/{}".format(randint(0, writes-1))
        f = open(path, "r")
        f.read()
