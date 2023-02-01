/* A integer to Roman conversion. */

/* Accepts an integer (u32).
Converts the integer to Roman literal and outputs it.
*/

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
