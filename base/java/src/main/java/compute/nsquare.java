package compute;

import java.util.logging.*;
import redis.clients.jedis.Jedis; 

public class Compute {

    void compute(){
        // size of the array to compute
        int size = Integer.parseInt(System.getenv("NSQUARE_ARRAY_SIZE"));
        int[] input = new int[size];

        for (int i=0; i<size;i++) {
            for (int j=0; j<size;j++) {
                input[j]=(input[i] + j) / 2;
            }
        }
    }

}
