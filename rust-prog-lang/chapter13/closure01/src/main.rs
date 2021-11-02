use std::thread;
use std::time::Duration;

//模拟健身计划

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulate_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulate_random_number
    )
}

fn generate_workout(intensity: u32,random_number: u32) {
    //let expensive_result = simulated_expensive_calculation(intensity);
    /*let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowingly...");
        thread::sleep(Duration::from_secs(2));
        num
    };*/
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            //simulated_expensive_calculation(intensity)
            //expensive_result
            //expensive_closure(intensity)
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps",

            //simulated_expensive_calculation(intensity)
            //expensive_result
            //expensive_closure(intensity)
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today,run for {} minutes",
                //simulated_expensive_calculation(intensity)
                //expensive_result
                //expensive_closure(intensity)
                expensive_result.value(intensity)
            );

        }
    }
}

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self,arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation) (arg);
                self.value = Some(v);
                v
            },
        }
    }
}