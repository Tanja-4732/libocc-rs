pub trait Projector<T> {
  fn project(&self) -> Vec<T>;
}
