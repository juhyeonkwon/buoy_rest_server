pub mod routes;


mod tests {
  use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

  #[test]
  fn test123421341() {
      let rand_string: String = thread_rng()
          .sample_iter(&Alphanumeric)
          .take(30)
          .map(char::from)
          .collect();

      println!("{}", rand_string);
  }
}