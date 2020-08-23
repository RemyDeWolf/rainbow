import os

def compute():
    # size of the array to sort
    size = int(os.environ['ARRAY_SIZE'])

    # create an input array with unsorted values
    arr = [0] * size
    incr = 1
    for i in range(size):
        arr[i] = -arr[i-1] + incr
        incr = -incr

    # sort the array
    arr.sort()
