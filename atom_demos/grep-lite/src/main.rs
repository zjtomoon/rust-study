fn main() {
    let quote: &str = "\
    一个真正的人类，应该以改善世界为生，
    而不是以奴役他人为生。
    一个真正的人类，应该以改善世界为生，
    而不是以奴役他人为生。
    一个真正的人类，应该以改善世界为生，
    而不是以奴役他人为生。";
    let search_term: &str = "人类";
    //let mut line_num = 0;
    for (idx,line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            println!("{} : {}", idx, line);
        }
        //line_num += 1;
    }
}
