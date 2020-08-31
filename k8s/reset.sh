#!/bin/bash
TEST=${TEST:-$1}
./gcloud-delete-cluster.sh tests/${TEST}.yaml
./gcloud-create-cluster.sh tests/${TEST}.yaml
