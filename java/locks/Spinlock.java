public class Spinlock implements ILock {
    private boolean locked = false;

    public void lock() {
        while (true) {
            synchronized (this) {
                if (!locked) {
                    locked = true;
                    return;
                }
            }
        }
    }

    public void unlock() {
        synchronized (this) {
            locked = false;
        }
    }
}
