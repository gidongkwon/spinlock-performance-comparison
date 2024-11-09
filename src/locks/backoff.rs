use std::{sync::atomic::{AtomicBool, Ordering}, thread, time::Duration};

use super::Lock;

pub struct BackoffLock {
  flag: AtomicBool,
}

impl BackoffLock {
  fn new() -> Self {
      BackoffLock {
          flag: AtomicBool::new(false),
      }
  }
}

impl Lock for BackoffLock {
  fn new() -> Self {
      BackoffLock::new()
  }

  fn lock(&self) {
      const MIN_DELAY: u64 = 5;
      const MAX_DELAY: u64 = 1000;

      let mut delay = MIN_DELAY;
      let ptr = self.flag.as_ptr();
      loop {
          unsafe {
              while *ptr {}
          }
          if self.flag.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) == Ok(false) {
              return;
          }
          thread::sleep(Duration::from_millis(fastrand::u64(0..1000) % delay));
          if delay < MAX_DELAY {
              delay *= 2;
          }
      }
  }

  fn unlock(&self) {
      self.flag.store(false, Ordering::Release);
  }
}