// #[macro_use]
// extern crate rocket;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

// #[launch]
// fn rocket() -> _ {
//     let a = my_first();
//     // let b = *a;

//     rocket::build().mount("/", routes![index])
// }

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            counter *= 2;
            break counter % counter;
        }
    };

    println!("The result is {result}");
}

// fn my_first() -> &'static str {
//     "my first"
// }
