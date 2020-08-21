package compute;

import java.util.logging.*;
import redis.clients.jedis.Jedis;
import java.util.concurrent.Executors;
import java.util.concurrent.ThreadPoolExecutor;

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
        int workers = Integer.parseInt(System.getenv().getOrDefault("WORKERS", "1"));

        if (workers<=1) {
            runCompute(0, batchSize, jedis, key);
        }else{
            ThreadPoolExecutor executor = (ThreadPoolExecutor) Executors.newFixedThreadPool(workers);
            for (int i = 1; i <= workers; i++) {
                Task task = new Task(i, batchSize, jedis, key);
                executor.execute(task);
            }
            executor.shutdown();
        }
    }

    public class Task implements Runnable {
        private int n;
        private int batchSize;
        private Jedis jedis;
        private String key;

        public Task(int n, int batchSize, Jedis jedis, String key) {
            this.n = n;
            this.batchSize = batchSize;
            this.jedis = jedis;
            this.key = key;
        }
        public void run() {
            runCompute(n, batchSize, jedis, key);
        }
    }
    private void runCompute(int n, int batchSize, Jedis jedis, String key){
        LOGGER.info(String.format("Starting worker [%s]", n));
        int count = 0;
        while (true){
            compute.compute();
            count++;
            if (count%batchSize == 0){
                jedis.incrBy(key, batchSize);
                LOGGER.info(String.format("[%s] Computed %s operations", n, count));
            }
        }
    }
}
