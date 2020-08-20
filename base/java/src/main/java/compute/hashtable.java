package compute;

import java.util.logging.*;
import java.util.Hashtable;
import java.util.Map;
import java.util.UUID;
import java.util.concurrent.ThreadLocalRandom;
import redis.clients.jedis.Jedis; 

public class Compute {

    void compute(){
        int size = Integer.parseInt(System.getenv("HASHTABLE_SIZE"));
        int readCount = Integer.parseInt(System.getenv("READ_COUNT"));

        // create an input map and an array
        Map<UUID, UUID> hashtable = new Hashtable<UUID, UUID>(); 
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

}
