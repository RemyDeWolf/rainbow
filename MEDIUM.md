<p align="center">
  <img src="/img/rainbow.png">
</p>

# Introducing Rainbow: Compare the performance of different programming languages

Recently, I was on the job market. For a software engineer, it means reviewing algorithms and deciding on which programming language(s)  to focus for your interviews. I used a platform called Leetcode to practice my rusty coding skills.  

How does it Leetcode work? For every question, it allows you to code a solution and submit it. It then runs it against a set of test cases to verify that the implementation is correct. It also gives you some information about runtime and memory and how your solution compares against others. These are very popular metrics and a lot of users would mention the stats of their algorithms in the Leetcode discussions.  

> Python easy solution, speed beats 99.32% and memory beats 99.78%  

If you happen to solve the exercises using different languages, you might be surprised by some outputs.  

Let's take a popular exercise: two-sum. It can be solved efficiently by using a hash table. Here is a valid solution in go.  
```go
// two-sum solution using Go
func twoSum(nums []int, target int) []int {
    
    mapSum:=make(map[int]int, 0)
    for i, v:=range nums{
        if j, ok:=mapSum[v]; ok{
            return []int{j, i}
        }
        mapSum[target-v]=i
    }
    return nil
}
```
Now let's submit this solution.  

<p align="center">
  <img src="/img/two-sum-go-submit.png" width="950">
</p>  

Sounds fast! Let's write a similar implementation in Python3 and submit it:  

<p align="center">
  <img src="/img/two-sum-python-submit.png" width="950">
</p>  

A bit slower but someway expected, python is an interpreted language.  
Let's try java next:  

<p align="center">
  <img src="/img/two-sum-java-submit.png" width="950">
</p>  

OMG! Java is so fast!  

Let's recap the results:  

| Language | Runtime | Memory |
| --- | --- | --- |
| Go | 20 ms | 6.5 MB |
| Python3 | 32 ms | 15.5 MB |
| Java | 3 ms | 42 MB |

  
To summarize:
* Java is **7x faster than Go**, and **10x faster than Python**.
* Java uses 3x more memory than Python, and 6x more than Go.
  
Is Java really that fast? The raw results could be a bit surprising, no? We are getting into interesting territory, **how do programming languages perform against each other?**  

Let's take a step back. Imagine, you have a PaaS to compute two-sum and you spend millions every year on cloud computing and you run your services on Python. Would that mean that you could lower your bill by 10x if you switch to Java? Well things like storage or network would be the same but you could save on the compute cost (AWS EC2, GCE, Azure VM). If you can try out different languages for your computing needs, you might be able to make a choice that would significantly reduce your VM costs.  

But how would you collect the data to make that decision?  

## Programming language & containers

Comparing programming languages performance is nothing new. But the common practice is not to use Docker containers to not impact the results due to containerization.  

But what if a language is exceptionally fast and robust but slow and unreliable when running in a container? What if the target infrastructure is Kubernetes?  

There are a lot of parameters that can impact performance when running an application in a container:  

* **Startup time**: The number of replicas can be scaled up or down to adjust based on traffic. If a container is slow to start, it would impact the throughput.
* **Image size**: A new node would need to download the image at least once for every version. The time to download the image is not spent processing requests.
* **Resiliency**: If a container crashes, a new one would need to be created to replace it.
* **CPU/Memory footprint**: The lighter the container is, the more can potentially be executed on the same node.
* **Language type**: compiled or interpreted. Running machine code is faster than running through a program line by line.

This list is not exhaustive which makes assessing programming language performance in a container a very hard question to answer.

## Introducing rainbow

If every characteristic of a containerized application is defined by a color, the idea of rainbow is to look at the full spectrum and to only compare the output. rainbow is an extensible framework to test and compare similar implementations running on containers. These implementations can be written in different languages. rainbow runs the containers on similar infrastructure and only focuses on the throughput.  

rainbow is an open source project hosted on Github under the MIT license.

## How does rainbow work?

The idea is simple, for each language, there is a main class which calls the function to test in an infinite loop. A batch size X is set, and every X call, a Redis counter is incremented by X to keep track of the number of computations done by each language.  

Every image is deployed on a similar node (same VM type) and can run as many replicas as needed to maximize the throughput.
This architecture provides a fair ground and rewards a language that can maximize the properties of containerization.  
<p align="center">
  <img src="/img/architecture.png">
</p>  

For more info about the testing methodology, check the project [readme](/README.md#how-does-it-work).

## Assumptions

Let's come back to the 3 programming languages that we used for two-sum.
* Java: This was the fastest language on Leetcode. Because it requires a Java Virtual Machine (JVM), the docker image should be bigger. This might impact performance.
* Go: This language produces binary executables, so we might be able to minimize the docker image. Being light, might reduce the gap with Java.
* Python: Python in docker is notoriously large, the default image uses linux buster (Debian). Also, it is an interpreted language, most likely it will perform poorly against the competition. Anything else would be a surprise.

## Implement two-sum in rainbow

Since we want to test the performance of the two-sum algorithm,  how can we write a program that would maximize the runtime doing that computation?
Let's keep it simple:
1. Implement the two-sum function exactly like it was defined in Leetcode.
1. Call this function with the same test case for each language. We can reuse one of the most complex test case used by Leetcode.

See the two-sum code for [Go](/base/go/twosum.go), [Python](/base/python/twosum.py), [Java](/base/java/src/main/java/compute/twosum.java).

## Building the docker images

Now, that we have the source code, let's package them in a docker image and build them. Let's take a moment to look at the image generated.  
* **Go image: 4.49 MB** Since Go produces a binary we can use a multistage build and copy the binary to an alpine image. (see Dockerfile)
* **Java image: 64.6 MB** The JDK takes 61 MB to install on alpine. (see Dockerfile)
* **Python image: 325.04 MB** The default python base image is 320 MB. There is an alpine image but it's not recommended as it can lead to weird production bugs. (see Dockerfile)

## Benchmarking in Kubernetes

Now that we have our docker images, we can deploy them to a Kubernetes cluster. For simplicity and affordability, we will use Google Kubernetes Engine (GKE) and create 3 nodes using the same machine type: e2-standard-2 (2xCPU, 8GB). Each pod is assigned to a node (using node labels).  

Let's try multiple combinations of workers/replicas and for each run the benchmark for 10 minutes. Count is the number of times, the compute method was called.

| Lang | Workers | Replicas | CPU(cores) | MEMORY(bytes) | Count |
| --- | --- | --- | --- | --- | --- |
| go | 1 | 1 | 1108m | 6Mi | 40900 |
| go | 1 | 2 | 1898m | 12Mi | 38200 |
| go | 2 | 1 | 1883m | 5Mi | **52200** |
| java | 1 | 1 | 990m | 59Mi | 150840 |
| java | 1 | 2 | 1859m | 114Mi | **177780** |
| java | 2 | 1 | 1797m | 61Mi | 153060 |
| python | 1 | 1 | 998m | 9Mi | 25780 |
| python | 1 | 2 | 1923m | 16Mi | 26540 |
| python | 2 | 1 | 998m | 9Mi | 26820 |

A few comments:
* This is clearly a CPU intensive operation. Memory does not impact the overall throughput.
* Python is limited by [the Python Global Interpreter Lock which prevents a container to use more than 1 CPU](https://realpython.com/python-gil/). When running on 2 replicas, it used more CPU but the throughput was not improved unfortunately.
* Go had best results when using workers, this might be explained by the lower overhead of the go routines (lightweight threads).
* Java performed slightly better when using 2 replicas.

## Comparing with Leetcode results
 
Java performance when running on Leetcode seems a bit inflated but the overall the trend is verified. For a computation like two-sum, Java greatly outperforms Go and Python. We can note that Go reduced the gap with Java, possibly because of the lighter docker image and the use of go routines.

| Lang | LeetCode | Rainbow |
| --- | --- | --- |
| java vs go (Runtime) | 7x | 3.5x |
| java vs python (Runtime) | 10x | 6.5x |
| java vs go (Memory) | 3x | 12x |
| java vs python (Memory) | 6x | 6.5x |

## Is Java really that fast?
Not so … fast. As we have seen in this exercise, the memory footprint of Go is much lower than Java. If memory was the limiting factor, we would have been able to run more go containers on the same node and potentially outperform Java. According to [The Computer Language Benchmarks Game](https://benchmarksgame-team.pages.debian.net/benchmarksgame/index.html), the fastest language are usually C++ and Rust.  

Using rainbow, I ran some benchmarks to compare a few languages (Go, Java, Python, Ruby, Rust) on generic implementations such as Hashtable, File IO, Sort, N-square and got various results. Rust, Java performed very well. Go did well but had a few weaknesses. The compiled languages outperformed the interpreted languages each time. All the results can be seen on the [project main page](/rainbow#implementations).  

This framework is extensible. If you have a specific implementation and would like to compare the performance of different languages, feel free to implement the compute function and have fun with it. See the instructions [here](/README.md#how-to-add-an-implementation).

## Further readings
* [Which programming language is fastest?](https://benchmarksgame-team.pages.debian.net/benchmarksgame/index.html)
