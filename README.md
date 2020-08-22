# rainbow
<table>
<tr>
<td>
  Compare the performance of different programming languages
</td>
</tr>
</table>

## Why Rainbow?

How do you compare programming languages efficiency when working with containers and microservices?  

There are a lot of parameters that can impact performance when running an application in a container:
* Startup time
* Image size
* Resiliency
* CPU/Memory footprint
* Language type: compiled or interpreted

This list is not exhaustive and it's a very hard question to answer.  
  
**Would it be possible to have a common base and only change the language used for a similar implementation?**  
If every characteristic is a color, the idea of Rainbow is to look at the full spectrum and only compare the output.  

Rainbow is a framework to test and compare simple implementations running on containers.  
It is well suited to compare programming languages as it can build and deploy the various implementations and run on similar infrastructure to have comparable outputs.

## When using Rainbow?

* Deciding which language to use for micro services.
* Comparing different base docker images for one language.
* Comparing different algorithms for the same language when running on containers.

## How does it work?

Rainbow defines an image container for each language.  
Each image is a simple application running a **main** function, which calls a **compute** function. 

Here is the **main** function for python:

```python
def main():
    key = 'python-{}.computed'.format(base, os.environ['COMPUTE'])

    batchSize = int(os.environ['BATCH_SIZE'])
    count = 0
    while True:
        # the compute() function needs to be implemented
        compute.compute()
        count+=1
        if count%batchSize == 0:
            redisClient.incr(key, batchSize)
            logging.info('Computed {} operations'.format(count))
```

The goal is to call the compute function as many times as possible in the allocated time and increment a redis key. For best results, the time spent computing should take hundreds of ms or more, so the time to update the redis key can be ignored (a few ms). If the computation is fast, set a high number for batch size.

To test an implementation, it needs to be implemented in each language to benchmark.  

Example of a python function doing some array computation:

```python
def compute():
    size = int(os.environ['NSQUARE_ARRAY_SIZE'])
    input = [0] * size

    # n square read/write operation
    for i in range(size):
        for j in range(size):
            input[j]=(input[i] + j) / 2
```

Here is the same function implemented in go:

```go
func compute() {
	size, _ := strconv.Atoi(os.Getenv("NSQUARE_ARRAY_SIZE"))
	input := make([]int, size)

	// n square read/write operation
	for i := 0; i < size; i++ {
		for j := 0; j < size; j++ {
			input[j] = (input[i] + j) / 2
		}
	}
}
```

Rainbow packages the various implementations in docker images.  
These docker images can be deployed to Kubernetes. By using dedicated worker nodes with identical specifications, we can compare the performance of the language.  

Every time a computation is done, a redis key is incremented.  
The values of these keys measure the language efficiency when running on containers.  
  
<p align="center">
  <img src="/img/architecture.png">
</p>
  
To recap, here are the various components:

| Name | Diff | Comments |
| --- | --- | --- |
| main function | language specific | This function keeps calling the compute function and increments the redis key. Already implemented in the base image. |
| compute function | language specific | This function needs to be implemented. Most of the time is spent here. |
| docker image | minimal | Each docker image uses the recommended base image for the language. It can also be changed for testing. |
| test configuration | identical | Defines application properties and infrastrucure (instance type, number of nodes). The configuration is the global. |
| redis key | specific | Each image is associated with one and only key. It contains the number of times the compute method has been executed. |
| k8s node | identical | Each image is assigned to a pool of nodes using the same instance type. |
| Running time | identical | The containers are depoyed together and the results are fetched when the time allocated is over. |

## Rainbow score

The Rainbow score is a measure of the language efficiency for the implementation under test.  

The output of a test is a list of redis key values.  

| Redis Key | Value |
| --- | --- |
| my-impl-rust| 14705 |
| my-impl-java | 8785 |
| my-impl-go | 6315 |

If the test ran for 10 min, rust was able to do 14,705 computations during that time.  

The maximum value gets a score of 100, and the other scores are relative percentages to the maximum value.

Example:
| Language | Rainbow Score |
| --- | --- |
| rust| 100 |
| java | 60 |
| go | 43 |

If there were many different runs, the rainbow score is the average of each individual Rainbow score.

## Implementations

* [nsquare](/impl/nsquare): n square operation on an array.
* [hashtable](/impl/hashtable): hashtable random access.
* [rwfiles](/impl/rwfiles): write files and access them randomly.

## How to use rainbow?

### Run locally using docker-compose

1. Select the [implementation](/impl) to test
1. Run `docker-compose build` to build the images
1. Optional: Edit the `.env` to set the runtime properties
1. Run `docker-compose up` to start the containers

The results can be seen diretly on the console ouput or by refreshing the redis web tracker at http://0.0.0.0:5000/

Example of output:
```
java-nsquare.computed: 4200
go-nsquare.computed: 4120
rust-nsquare.computed: 4420
ruby-nsquare.computed: 60
python-nsquare.computed: 40
```

You can also customize the docker-compose file to test different base images for a language:
```
python:3.8-buster-nsquare.computed: 700
python:3.8-alpine-nsquare.computed: 660
python:3.8-slim-buster-nsquare.computed: 710
```

### Run on kubernetes

See [Run the rainbow tests in the cloud](/k8s/README.md).

## How to add an implementation?

1. Define the implementation name, this name is used with the files to implement.
1. Create a directory `/impl/$name`
    * Add a readme following this [example](/impl/nsquare).
    * Add a docker-compose file following this [example](/impl/nsquare/docker-compose.yml).
1. For each language to test, implement the *compute* function:
    * go: `/base/go/$name.go`
    * java: `/base/java/src/main/java/compute/$name.java`
    * python: `/base/python/$name.py`
    * ruby: `/base/rust/src/$name.rs`
1. [Optional] Add kubernetes tests following this [example](k8s/tests/nsquare.yaml). If you have some test results, include them to the readme.
1. Send a Pull Request.

## [License](LICENSE)

MIT © [Remy DeWolf](https://github.com/RemyDeWolf)
