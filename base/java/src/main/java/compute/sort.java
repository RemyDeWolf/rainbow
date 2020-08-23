package compute;

import java.util.Arrays; 

public class Compute {

    void compute(){
        // size of the array to sort
        int size = Integer.parseInt(System.getenv("ARRAY_SIZE"));

        //create an input array with unsorted values
        int[] arr = new int[size];
        int incr = 1;
        for (int i=1; i<size;i++) {
            arr[i] = -arr[i-1] + incr;
            incr = -incr;
        }

        Arrays.sort(arr);
    }

}
