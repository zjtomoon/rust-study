fn main() {
    let quote: &str = "\
    一个真正的人类，应该以改善世界为生，
    而不是以奴役他人为生。
    一个真正的人类，应该以改善世界为生，
    而不是以奴役他人为生。
    一个真正的人类，应该以改善世界为生，
    而不是以奴役他人为生。";
    let search_term: &str = "人类";
    let ctx_len = 2;
    //let mut line_num = 0;
    // for (idx,line) in quote.lines().enumerate() {
    //     if line.contains(search_term) {
    //         println!("{} : {}", idx, line);
    //     }
    //     //line_num += 1;
    // }
    let mut tags: Vec<usize> = Vec::new();
    let mut ctx : Vec<Vec<(usize,String)>> = Vec::new();
    for (i,line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            tags.push(i);
            let v = Vec::with_capacity(2 * ctx_len + 1);
            ctx.push(v);
        }
    }
    if tags.len() == 0 {
        return;
    }
    for (i,line) in quote.lines().enumerate() {
        for (j,tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_len);
            let upper_bound = tag + ctx_len;
            if(i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i,line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for item in ctx.iter() {
        for &(i,ref line) in item.iter() {
            let line_num = i+1;
            println!("{}:{}",line_num,line);
        }
    }
}
