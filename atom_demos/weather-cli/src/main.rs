mod weather;
use exitfailure::ExitFailure;
use structopt::StructOpt;
use weather::W;
use chrono::{DateTime,Local,NaiveDateTime,Duration};


#[derive(StructOpt)]
struct Input {
    city: String
}

#[tokio::main]
async fn main() -> Result<(),ExitFailure> {
    let input = Input::from_args();
    let resp = W::get(&input.city).await?;

    let now:NaiveDateTime = Local::now().naive_local();
    println!("当前时间: {}",now);

    // TODO: 绝对温度转摄氏度
    println!("当前温度: {} °C\n 最高温度: {} °C\n 最低温度: {} °C\n 湿度: {}\n 大气压: {} hPa\n 体感温度: {} °C\n",
            // input.city,
             (resp.main.temp - 273.15) as i32,
             (resp.main.temp_max - 273.15) as i32,
             (resp.main.temp_min - 273.15) as i32,
             resp.main.humidity,
             resp.main.pressure,
             (resp.main.feels_like - 273.15) as i32
    );
    Ok(())
}




