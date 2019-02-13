fn main() {
  struct Gift {
    sequence: String,
    item: String
  };

  let gifts = [
    Gift { sequence: String::from("first"), item: String::from("a partridge in a pear tree") },
    Gift { sequence: String::from("second"), item: String::from("two turtledoves") },
    Gift { sequence: String::from("third"), item: String::from("three French hens") },
    Gift { sequence: String::from("fourth"), item: String::from("four calling birds") },
    Gift { sequence: String::from("fifth"), item: String::from("five gold rings") },
    Gift { sequence: String::from("sixth"), item: String::from("six geese a-laying") }
  ];

  let mut stanza = String::from("");

  for (index, gift) in gifts.iter().enumerate() {
    if index == 0 {
      stanza = gift.item.clone();
    } else if index == 1 {
      stanza = gift.item.clone() + " and " + &stanza;
    } else {
      stanza = gift.item.clone() + ", " + &stanza;
    }
    println!("On the {} day of Christmas, my true love gave to me {}.", gift.sequence, stanza);
  }
}