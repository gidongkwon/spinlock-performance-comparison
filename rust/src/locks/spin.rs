use std::sync::atomic::{AtomicBool, Ordering};

use super::Lock;

pub struct Spinlock {
  flag: AtomicBool,
}

impl Spinlock {
  fn new() -> Self {
      Spinlock {
          flag: AtomicBool::new(false),
      }
  }
}

impl Lock for Spinlock {
  fn new() -> Self {
      Spinlock::new()
  }

  fn lock(&self) {
      while self.flag.swap(true, Ordering::Acquire) {}
  }

  fn unlock(&self) {
      self.flag.store(false, Ordering::Release);
  }
}