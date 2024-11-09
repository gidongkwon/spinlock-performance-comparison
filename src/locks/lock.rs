pub trait Lock: Sync + Send {
  fn new() -> Self;
  fn lock(&self);
  fn unlock(&self);
}