# Testing various Python base images

When running nsquare with Python and comparing with other programming languages, python performed poorly: 100x slower than Go.  

The Python image uses `python:latest` by default, which as of August 2020 is equivalent to `3.8.5-buster`.  

There are many articles ([1](https://pythonspeed.com/articles/base-image-python-docker-images/), [2](https://superuser.com/questions/1219609/why-is-the-alpine-docker-image-over-50-slower-than-the-ubuntu-image)) that mention that using the Python alpine image is slower at runtime.  
To verify this, a specific run was done to compare various base images.

```
NSQUARE_ARRAY_SIZE=2000
BATCH_SIZE=20
DURATION=10min
MACHINE_TYPE=e2-standard-2 (2xCPU 8GB)
```

| Base Image | Count |
| --- | --- |
| 3.8-alpine | 1020 |
| 3.8-slim-buster | 1060 |
| 3.8-buster | 1040 |
