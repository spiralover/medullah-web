pub fn to_cent(float: f64) -> i64 {
    (float * 100.00).round() as i64
}

pub fn from_cent(int: i64) -> f64 {
    (int as f64) / 100.00
}

pub fn human_readable(num: f64) -> String {
    let mut num = num.to_string();
    let mut str_num = String::new();
    let mut negative = false;
    let values: Vec<char> = num.chars().collect();

    if values[0] == '-' {
        num.remove(0);
        negative = true;
    }

    for (i, char) in num.chars().rev().enumerate() {
        if i % 3 == 0 && i != 0 {
            str_num.insert(0, ',');
        }
        str_num.insert(0, char);
    }

    if negative {
        str_num.insert(0, '-');
    }

    str_num.replace(",.", ".")
}
