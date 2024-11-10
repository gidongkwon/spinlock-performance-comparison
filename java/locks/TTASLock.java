import java.util.concurrent.atomic.AtomicBoolean;

public class TTASLock implements ILock {
  AtomicBoolean flag = new AtomicBoolean(false);

  @Override
  public void lock() {
    while (true) {
      while (flag.get()) {}
      if (!flag.getAndSet(true)) {
        return;
      }
    }
  }

  @Override
  public void unlock() {
    flag.set(false);
  }
}
