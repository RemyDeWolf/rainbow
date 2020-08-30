#!/bin/bash
# Run the tests as defined in the tests/$name.yaml file.
# 1. Deploy redis 
# 2. Create the test deployments
# 3. Wait for the test duration (defined in `images:`)
# 4. Delete the test deployments
# 5. Report the results by fetching the redis data

. .yaml-parser.sh

if [ -z "$1" ] || [ ! -f "$1" ]; then
    echo "script usage: $(basename $0) tests/application.yaml"
    exit 1
fi

yaml_file=$1

# make sure redis is deployed
echo "Deploying redis"
kubectl apply -f "manifests/"
while [[ $(kubectl get pods -l app=redis -o 'jsonpath={..status.conditions[?(@.type=="Ready")].status}') != "True" ]]; do echo "waiting for redis pod" && sleep 1; done
echo "Redis deployed. To see the data, run 'kubectl port-forward deployment/redis-web 5000 5000'"
echo "And go to http://0.0.0.0:5000/"

# flush redis keys before running the tests
echo "Flushing all redis keys"
kubectl exec -it deployment/redis redis-cli flushall

# create the deployments for the images to test
type=$(get_value $yaml_file type pod)
echo "Creating the test ${type}s"
helm template -f $yaml_file . |  kubectl create -f -

if [ "$type" = "job" ]; then
    echo "jobs started, exiting"
    exit 0
fi

duration_sec=$(get_value $yaml_file duration_sec 600)
echo `date +"%r"` "Running the tests for $duration_sec sec"
sleep $duration_sec
echo `date +"%r"` "Tests completed"

# delete the deployments
echo "Deleting the test deployments"
helm template -f $yaml_file . |  kubectl delete -f -

# output all the redis keys and their values
echo "Dumping redis data:"
kubectl exec -t deployment/redis -- /bin/sh -c "redis-cli keys \* | while read -r key ; do redis-cli GET \$key | xargs -I % sh -c \"echo \$key %\" ; done"
