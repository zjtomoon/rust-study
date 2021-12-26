use regex::Regex;

fn main() {
    let re = Regex::new("人类").unwrap();
    let text = "\
    一个真正的人类，应该以改善世界为生，
    而不是以奴役他人为生。
    一个真正的人类，应该以改善世界为生，
    而不是以奴役他人为生。
    一个真正的人类，应该以改善世界为生，
    而不是以奴役他人为生。 ";
    for line in text.lines() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(_) => println!("{}",line),
            None => ()
        }
    }
}
