fn main() {
  let gifts = [
    ("first", "a partridge in a pear tree"),
    ("second", "two turtledoves"),
    ("third", "three French hens"),
    ("fourth", "four calling birds"),
    ("fifth", "five gold rings"),
    ("sixth", "six geese a-laying")
  ];

  let mut stanza = String::from("");

  for index in 0..gifts.len() {
    stanza = String::from(gifts[index].1) + &String::from(", ") + &stanza;
    println!("On the {} day of Christmas, my true love gave to me {}.", gifts[index].0, stanza);
  }
}