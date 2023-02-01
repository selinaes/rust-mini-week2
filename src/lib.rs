/* A integer to Roman conversion. */

/* Accepts an integer (u32).
Converts the integer to Roman literal and outputs it.
*/
// add necessary dependencies
use serde_json;
use rand::Rng;

// a function that reads data from a json file and returns a string
pub fn interpret(num: i32) -> String {
    let data = include_str!("tarot-images.json");
    let v: serde_json::Value = serde_json::from_str(data).unwrap();
    let mut result: String = "".to_owned();

    let card = &v[num.to_string()];
    let name = &card["name"];
   
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..2);
    let fortune= &card["fortune_telling"][random.to_string()];

    let name = name.as_str().unwrap();
    result.push_str(name);
    result.push_str(" ");
    result.push_str(num.to_string().as_str());
    result.push_str(": ");
    let fortune: &str = fortune.as_str().unwrap();
    result.push_str(fortune);

    result
}




// a function that takes in an integer and retur
pub fn convert(num: &u32) -> String {
    let thousands: Vec<&str> = vec!["", "M", "MM", "MMM"];
    let hundreds: Vec<&str> = vec!["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    let tens: Vec<&str> = vec!["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    let ones: Vec<&str> = vec!["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    let sizenum = *num as usize;
    let mut result: String = "".to_owned();
    result.push_str(thousands[sizenum / 1000]);
    result.push_str(hundreds[sizenum % 1000 / 100]);
    result.push_str(tens[sizenum % 100 / 10]);
    result.push_str(ones[sizenum % 10]);

    result
}

// a function that draws a random number between 0 and 57
pub fn draw() -> String {
    let mut rng = rand::thread_rng();
    let random:i32 = rng.gen_range(0..57);

    interpret(random)

}
