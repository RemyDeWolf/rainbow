package compute;

import java.util.logging.*;
import java.io.File;
import java.io.FileOutputStream;
import java.nio.file.Paths;
import java.nio.file.Files;
import java.util.concurrent.ThreadLocalRandom;


public class Compute {

    void compute() throws Exception {
        int writes = Integer.parseInt(System.getenv("WRITES"));
        int reads = Integer.parseInt(System.getenv("READS"));
        int fileSize = Integer.parseInt(System.getenv("FILE_SIZE"));

        //create the data array
        byte[] data = new byte[fileSize];
        
        // write the files
        for (int i=0; i<writes; i++) {
            String path = String.format("/data/%s;", i);
            try (FileOutputStream stream = new FileOutputStream(path)) {
                stream.write(data);
            }
        }

        //read the files
        for (int i=0; i<reads; i++) {
            int index = ThreadLocalRandom.current().nextInt(0, writes);
            String path = String.format("/data/%s;", index);
            Files.readAllBytes(Paths.get(path));
        }
    }

}
