#!/bin/bash
# Delete the k8s cluster and associated resources

. .yaml-parser.sh

if [ -z "$1" ] || [ ! -f "$1" ]; then
    echo "script usage: $(basename $0) tests/application.yaml"
    exit 1
fi

yaml_file=$1

region=$(get_value $yaml_file region us-west1)
cluster_name=$(get_value $yaml_file cluster_name rainbow-cluster)

echo "Deleting the cluster $cluster_name"
gcloud container clusters delete $cluster_name --region=$region -q
