use std::sync::atomic::{AtomicBool, Ordering};

use super::Lock;

pub struct TASLock {
    flag: AtomicBool,
}

impl TASLock {
  fn new() -> Self {
      TASLock {
          flag: AtomicBool::new(false),
      }
  }
}

impl Lock for TASLock {
  fn new() -> Self {
      TASLock::new()
  }

  fn lock(&self) {
      while self.flag.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) != Ok(false) {}
  }

  fn unlock(&self) {
      self.flag.store(false, Ordering::Release);
  }
}