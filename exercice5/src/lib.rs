fn main() {
  println!("Hello, world 5!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(3, 2);
  }
}
