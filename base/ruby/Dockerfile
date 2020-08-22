ARG BASE=ruby:latest
FROM $BASE

RUN gem install redis concurrent-ruby
COPY main.rb .

# copy the script to compute
ARG COMPUTE
COPY $COMPUTE.rb ./compute.rb
ENV COMPUTE=$COMPUTE

ARG BASE
ENV BASE=$BASE
RUN mkdir /data

ENTRYPOINT ["ruby", "main.rb"]
