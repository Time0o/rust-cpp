mod integer_rust;

fn main() {
  println!("WTF");
  let mut i = integer_rust::Integer::new(1);
  println!("{}", i.get());
  i.set(2);
  println!("{}", i.get());
}
