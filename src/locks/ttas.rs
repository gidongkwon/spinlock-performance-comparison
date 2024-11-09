use std::sync::atomic::{AtomicBool, Ordering};

use super::Lock;

pub struct TTASLock {
  flag: AtomicBool,
}

impl TTASLock {
  fn new() -> Self {
      TTASLock {
          flag: AtomicBool::new(false),
      }
  }
}

impl Lock for TTASLock {
  fn new() -> Self {
      TTASLock::new()
  }

  fn lock(&self) {
      let ptr = self.flag.as_ptr();
      loop {
          unsafe {
              while *ptr {}
          }
          if self.flag.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) == Ok(false) {
              return;
          }
      }
  }

  fn unlock(&self) {
      self.flag.store(false, Ordering::Release);
  }
}