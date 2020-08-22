
FROM golang:alpine AS builder
WORKDIR /go/src/app

COPY go.mod go.sum main.go ./
# copy the script to compute
ARG COMPUTE
COPY $COMPUTE.go ./compute.go

# RUN go mod download
RUN go mod tidy

ARG VERSION=dev
RUN GOOS=linux GOARCH=amd64 go build -ldflags="-w -s" -o /go/bin/main *.go


FROM alpine:latest

# set environment variables
ARG COMPUTE
ENV USER_UID=1001 \
  USER_NAME=go \
  COMPUTE=$COMPUTE

# install the binary built in previous stage
COPY --from=builder /go/bin/main /usr/local/bin/main

# set entrypoint
ENTRYPOINT ["/usr/local/bin/main"]

# directory for application files
RUN mkdir /data
RUN chown -R $USER_UID /data

# switch to non-root user
USER ${USER_UID}

