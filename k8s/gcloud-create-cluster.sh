#!/bin/bash
# Create the k8s cluster and the node pool(s) for each image

. .yaml-parser.sh

if [ -z "$1" ] || [ ! -f "$1" ]; then
    echo "script usage: $(basename $0) tests/application.yaml"
    exit 1
fi

yaml_file=$1

region=$(get_value $yaml_file region us-west-1)
#e2-standard-2 cost-optimized machine 2xCPU 8GB
machine_type=$(get_value $yaml_file machine_type e2-standard-2)
num_nodes=$(get_value $yaml_file num_nodes 1)
cluster_name=$(get_value $yaml_file cluster_name rainbow-cluster)
images=$(get_values $yaml_file images)

if [ -z "$images" ]; then
    echo "'images' not defined in $yaml_file"
    exit 1
fi

# create a cluster
echo "Creating the cluster $cluster_name"
# just use 1 node for the master + redis, applications have their own node pools
gcloud container clusters create $cluster_name \
    --region=$region \
    --node-labels=runner=main \
    --num-nodes=1

# create a node pool with n node(s) for each image to test
for image in $images; do
    #image: .../rainbow:nsquare-go
    #app: nsquare-go
    app=${image##*:}
    # '.' is not allowed in node pool name
    name=${app//./-}
    echo "Creating node pool $cluster_name with $num_nodes $machine_type"
    gcloud container node-pools create $name \
    --region=$region \
    --cluster=$cluster_name \
    --node-labels=runner=$name \
    --machine-type=$machine_type \
    --num-nodes=$num_nodes
done

echo "Cluster $cluster_name created successfully"
