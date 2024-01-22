fn main() {
  let message: &str = "The temperature today is:";
  let x: [&str; 2] = [message, 100];
  println!("{} {}", x[0], x[1]);
}
