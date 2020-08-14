import os

def compute():
    size = int(os.environ['NSQUARE_ARRAY_SIZE'])
    input = [0] * size

    # n square read/write operation
    for i in range(size):
        for j in range(size):
            input[j]=(input[i] + j) / 2
