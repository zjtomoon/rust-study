use exitfailure::ExitFailure;
mod weather;
use structopt::StructOpt;
use weather::W;

#[derive(StructOpt)]
struct Input {
    city: String
}
#[tokio::main]
async fn main() -> Result<(),ExitFailure>{
    let input = Input::from_args();
    let resp = W::get(&input.city).await?;

    // TODO: 绝对温度转摄氏度
    println!("{} \n 当前温度： {} °C\n 最高温度： {} °C\n 最低温度： {} °C\n 湿度： {}\n 大气压: {} hPa\n 体感温度: {} °C\n",
             input.city,
             (resp.main.temp - 273.15) as i32,
             (resp.main.temp_max -273.15) as i32,
             (resp.main.temp_min - 273.15) as i32,
             resp.main.humidity,
             resp.main.pressure,
             (resp.main.feels_like - 273.15) as i32
    );
    Ok(())
}




