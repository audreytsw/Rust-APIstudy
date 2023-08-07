#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/fufi/<name>/<happy>")]
fn fufi(name: String, happy: bool) -> String {
    if happy {
        format!("Hello, Penguin!, {} is happy!", name)
    } else { 
        format!("Hello, Penguin!, {} is sad", name)
    }
}

#[get("/age/<age>")]
fn age(age: u8) -> String {
    if age > 5 {
        format!("Your age is {}, you're older than Fufi!", age)
    } else if age < 5 {
        format!("Your age is {}, you're younger than Fufi!", age)
    } else {
        format!("Your age is {}, Fufi is your friend!", age)
    }
}

fn main(){
    rocket::ignite().mount("/", routes![index, fufi, age]).launch();
    
}
