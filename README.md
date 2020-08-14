# rainbow
<table>
<tr>
<td>
  Compare the performance of different programming languages
</td>
</tr>
</table>

## Implementations

* [nsquare](/impl/nsquare): n square operation on an array.

## How to use?

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
ruby:alpine-nsquare.computed: 60
python:3.8-alpine-nsquare.computed: 40
```

You can also customize the docker-compose file to test different base images for a language:
```
python:3.8-buster-nsquare.computed: 700
python:3.8-alpine-nsquare.computed: 660
python:3.8-slim-buster-nsquare.computed: 710
```

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
1. Test and send a Pull Request.

## [License](LICENSE)

MIT © [Remy DeWolf](https://github.com/RemyDeWolf)
