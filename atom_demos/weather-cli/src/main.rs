use exitfailure::ExitFailure;
use structopt::StructOpt;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

#[derive(StructOpt)]
struct Input {
    city: String
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let input = Input::from_args();
    println!("{}", input.city);
    Ok(())
}


// TODO: 1、汉字转拼音
// TODO: 2、绝对温度转摄氏度
// TODO: 3、天气预报


