ARG BASE=ruby:latest
FROM $BASE

RUN gem install redis
COPY main.rb .

# copy the script to compute
ARG COMPUTE
COPY $COMPUTE.rb ./compute.rb
ENV COMPUTE=$COMPUTE

ARG BASE
ENV BASE=$BASE

ENTRYPOINT ["ruby", "main.rb"]
