fn main() {
  let gifts = [
    ("first", "a partridge"),
    ("second", "two turtledoves, and"),
    ("third", "three French hens")
  ];

  // let stanza = "";

  for index in 0..gifts.len() {
    println!("On the {} day gave me ", gifts[index].0);
    for segment in (0..(index + 1)).rev() {
      println!("{}, ", gifts[segment].1);
    }
  }
}