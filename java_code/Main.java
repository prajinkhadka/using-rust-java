import java.io.*;

public class Main {
    public static void main(String[] args) {
        String filePath = "../test_inputs/twitter_large.json";
        long start = System.nanoTime();

        String result = RustJsonParser.parseJson(filePath);

        long end = System.nanoTime();
        long totalTime = end - start;

        System.out.println("\nResult: " + result);
        System.out.printf("Total time: %.2f ms\n", totalTime / 1_000_000.0);
    }
}
