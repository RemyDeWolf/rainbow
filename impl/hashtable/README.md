# hashtable

Generate a [Hash table](https://en.wikipedia.org/wiki/Hash_table) and access it randomly.

## Implementations

* [go](/base/go/hashtable.go)
* [java](/base/java/src/main/java/compute/hashtable.java)
* [python](/base/python/hashtable.py)
* [ruby](/base/ruby/hashtable.rb)
* [rust](/base/rust/src/hashtable.rs)

## Example of implementation

```java
    void compute(){
        int size = Integer.parseInt(System.getenv("HASHTABLE_SIZE"));
        int readCount = Integer.parseInt(System.getenv("READ_COUNT"));

        // create an input map and an array
        Map<UUID, UUID> hashtable = new Hashtable<UUID, UUID>(size, 1.0);
        UUID[] keys = new UUID[size];

        // init the map
        for (int i=0; i<size; i++) {
            UUID key = UUID.randomUUID();
            hashtable.put(key, key);
            keys[i] = key;
        }

        // access map
        for (int i=0; i<readCount;i++) {
            int index = ThreadLocalRandom.current().nextInt(0, size);
            UUID key = keys[index];
            hashtable.get(key);
        }
        
    }
```
