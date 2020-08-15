ARG BASE=python:latest
FROM $BASE
WORKDIR /code

COPY requirements.txt requirements.txt
RUN pip install -r requirements.txt
COPY main.py requirements.txt ./
# copy the script to compute
ARG COMPUTE
COPY $COMPUTE.py ./compute.py
ENV COMPUTE=$COMPUTE

ARG BASE
ENV BASE=$BASE

CMD ["python3", "main.py"]
