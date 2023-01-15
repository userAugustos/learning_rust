mod characters;
use characters::characters;

fn main() {
  println!("Wellcome to the game, select your character: ");
  
  for character in characters.iter() {
    println!("{:?}", character)
  }
}