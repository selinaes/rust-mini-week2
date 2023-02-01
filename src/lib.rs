/* A Tarot draw & interpretation. */

use rand::Rng;

// a function that reads data from a json file and returns a string
pub fn interpret(num: i32) -> String {
    let data = include_str!("tarot-images.json");
    let v: serde_json::Value = serde_json::from_str(data).unwrap();
    let mut result: String = "".to_owned();

    let cards = &v["cards"].as_array().unwrap();
    let card = cards[num as usize].as_object().unwrap();
    let name = card["name"].as_str().unwrap();

    let fortunes = card["fortune_telling"].as_array().unwrap();
    let f_size = fortunes.len();
    let mut random: usize = 0;
    if f_size > 1 {
        let mut rng = rand::thread_rng();
        random = rng.gen_range(0..f_size - 1);
    }
    let fortune: &str = fortunes[random].as_str().unwrap();

    result.push_str(name);
    result.push(' ');
    result.push_str(num.to_string().as_str());
    result.push_str(": ");

    result.push_str(fortune);

    result
}

// a function that draws a random number between 0 and 57
pub fn draw() -> String {
    let mut rng = rand::thread_rng();
    let random: i32 = rng.gen_range(0..57);

    interpret(random)
}
