package compute;

import java.util.logging.*;
import redis.clients.jedis.Jedis; 

public class Main {
    private static Logger LOGGER = null;

    static {
        System.setProperty("java.util.logging.SimpleFormatter.format",
        "[%1$tF %1$tT] [%4$-7s] %5$s %n");
        LOGGER = Logger.getLogger(Main.class.getName());
    }

    private Compute compute = new Compute();

    public static void main(String[] args) {
        Main main = new Main();
        main.run();
    }

    private void run(){
        String key = String.format("java-%s.computed", System.getenv("COMPUTE"));
        LOGGER.info(String.format("Redis key: %s",key));

        Jedis jedis = new Jedis("redis");

        int batchSize = Integer.parseInt(System.getenv("BATCH_SIZE"));
        int count = 0;
        while (true){
            compute.compute();
            count++;
            if (count%batchSize == 0){
                jedis.incrBy(key, batchSize);
                LOGGER.info(String.format("Computed %s operations", count));
            }
        }
    }
}
