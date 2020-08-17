# Run the rainbow tests in the cloud

## Define the test configuration file

Add a yaml file following this [example](tests/nsquare.yaml).  
Or reuse an existing configuration.

## Create the Kubernetes cluster

To deploy the containers, you will need a Kubernetes cluster.  
To be able to compare the results, the pods using the same image are only able to run on particular node(s).  

For example if testing `nsquare-go` and `nsquare-java`, the cluster would have 3 nodes:
- main node: k8s api pods, `redis`, `redis-web`
- 2nd node: reserved for `nsquare-go` using a pod selector with label `runner=nsquare-go`
- 3rd node: reserved for `nsquare-java` using a pod selector with label `runner=nsquare-java`

Each reserved nodes will host the same number of `replicas` defined in the test configuration.  
After each computation, the main function increments a counter on `redis`.  
At any time, the redis data can be viewed with `redis-web`.

### Using Google Kubernetes Engine (GKE)

This one has been fully scripted, make sure you have completed the [GKE quickstart](https://cloud.google.com/kubernetes-engine/docs/quickstart) first.  


This script will create the GKE cluster and the node pools for each image to test.
```
./gcloud-create-cluster.sh tests/nsquare.yaml
Creating the cluster rainbow-cluster
...
Creating cluster rainbow-cluster in us-west1-a... Cluster is being health-checked (master is healthy)...done.      
Created [https://container.googleapis.com/v1/projects/rainbow-286323/zones/us-west1-a/clusters/rainbow-cluster].
...
Creating node pool rainbow-cluster with 1 e2-standard-2
...
Creating node pool nsquare-go...done.     

Creating node pool rainbow-cluster with 1 e2-standard-2
...
Creating node pool nsquare-java...done.      
...
Cluster rainbow-cluster created successfully
```

## Run the tests

Here are the steps done when running the tests.
1. Deploy redis 
1. For each image (defined in `images:`), deploy n replicas(defined in `replicas:`) on their reserved node(s) 
1. Wait for the test duration (defined in `images:`)
1. Delete the deployed applications
1. Report the results by fetching the redis data

To run the tests, run the following script using the test configuration file as parameter.

```
./run-tests.sh tests/nsquare.yaml

Deploying redis
...

Flushing all redis keys
...
Creating the test deployments
...
04:16:06 PM Running the tests for 600 sec
04:26:06 PM Tests completed
...
Deleting the test deployments
...
Dumping redis data:
rust-nsquare.computed 48
java-nsquare.computed 60
go-nsquare.computed 64
```

Because the redis data is flushed when starting the tests, you can do successive runs after tweaking the test properties in the configuration file.
