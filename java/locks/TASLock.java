import java.util.concurrent.atomic.AtomicBoolean;

public class TASLock implements ILock {
  AtomicBoolean flag = new AtomicBoolean(false);

  @Override
  public void lock() {
    while (flag.getAndSet(true)) {}
  }

  @Override
  public void unlock() {
    flag.set(false); 
  }
}
