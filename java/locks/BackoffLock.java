import java.util.concurrent.atomic.AtomicBoolean;

public class BackoffLock implements ILock {
  AtomicBoolean flag = new AtomicBoolean(false);
  static final int MIN_DELAY = 3;
  static final int MAX_DELAY = 1000;

  @Override
  public void lock() {
    int delay = MIN_DELAY;
    while (true) {
      while (flag.get()) {}
      if (!flag.getAndSet(true)) {
        return;
      }

      long random = (long)(Math.random() * (MAX_DELAY - MIN_DELAY) + MIN_DELAY);
      try {
        Thread.sleep(random % delay);
      } catch (InterruptedException e) {
        e.printStackTrace();
      }

      if (delay < MAX_DELAY) {
        delay *= 2;
      }
    }
  }

  @Override
  public void unlock() {
    flag.set(false);
  }
}
