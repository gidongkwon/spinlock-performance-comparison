import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class Main {
    private static final List<Integer> THREAD_COUNTS = Arrays.asList(2, 4, 8, 16, 32);
    private static final int REPEAT_COUNT = 10;
    private static final long START = 1_000_000;
    private static final long END = 5_000_000;
    private static final String SEP = "\t";

    public static void main(String[] args) {
        System.out.println("Lock Type" + SEP + THREAD_COUNTS.stream()
                .map(Object::toString)
                .collect(Collectors.joining(SEP)));
        
        noLock();
        benchmarkLock(new Spinlock(), "Spinlock");
        benchmarkLock(new TTASLock(), "TTASLock");
        benchmarkLock(new TASLock(), "TASLock");
        benchmarkLock(new BackoffLock(), "BackoffLock");
    }

    private static void benchmarkLock(ILock lock, String name) {
        System.out.print(String.format("%-12s%s", name, SEP));

        for (int threadCount : THREAD_COUNTS) {
            long timeAvg = 0;

            for (int j = 0; j < REPEAT_COUNT; j++) {
                final long[] sum = {0};
                long chunkSize = (END - START) / threadCount;
                
                long startTime = System.currentTimeMillis();

                Thread[] threads = new Thread[threadCount];
                for (int i = 0; i < threadCount; i++) {
                    long start = START + i * chunkSize;
                    long end = (i == threadCount - 1) ? END : START + (i + 1) * chunkSize - 1;
                    threads[i] = new Thread(() -> {
                        for (long k = start; k <= end; k++) {
                            lock.lock();
                            try {
                                sum[0] += k;
                            } finally {
                                lock.unlock();
                            }
                        }
                    });
                }

                for (Thread t : threads) t.start();
                for (Thread t : threads) {
                    try {
                        t.join();
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                }

                long duration = System.currentTimeMillis() - startTime;
                timeAvg += duration;
                // System.out.println(sum[0] + SEP);
            }

            timeAvg /= REPEAT_COUNT;
            System.out.print(timeAvg + SEP);
        }
        
        System.out.println();
    }

    private static void noLock() {
        System.out.print("No Lock" + SEP);

        for (int threadCount : THREAD_COUNTS) {
            final long[] sum = {0};
            long chunkSize = (END - START) / threadCount;
            long startTime = System.currentTimeMillis();

            Thread[] threads = new Thread[threadCount];
            for (int i = 0; i < threadCount; i++) {
                long start = START + i * chunkSize;
                long end = (i == threadCount - 1) ? END : START + (i + 1) * chunkSize - 1;
                threads[i] = new Thread(() -> {
                    for (long k = start; k <= end; k++) {
                        sum[0] += k;
                    }
                });
            }

            for (Thread t : threads) t.start();
            for (Thread t : threads) {
                try {
                    t.join();
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
            }

            long duration = System.currentTimeMillis() - startTime;
            System.out.print(duration + "," + sum[0] + SEP);
        }

        System.out.println();
    }
}