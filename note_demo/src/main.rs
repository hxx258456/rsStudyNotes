use note_demo::kinds::PrimaryColor;
use note_demo::utils::mix;

fn main() {
    let blue = PrimaryColor::Blue;
    let yellow = PrimaryColor::Yellow;
    println!("{:?}",mix(blue,yellow));
}
