#![allow(clippy::no_effect_underscore_binding)]

use std::process::Command;

#[macro_use]
extern crate rocket;

#[get("/cowsay/<input>/<kind>")]
fn cowsay_animal(input: &str, kind: &str) -> String {
    let cowsay_output = Command::new("cowsay").args(["-f", kind, input]).output();

    let response = match cowsay_output {
        Ok(out) => {
            if !out.stdout.is_empty() && !out.stderr.is_empty() {
                [out.stdout, out.stderr].concat()
            } else if out.stderr.is_empty() {
                out.stdout
            } else {
                out.stderr
            }
        }
        Err(err) => err.to_string().into(),
    };

    let response = std::str::from_utf8(&response);
    match response {
        Ok(res) => res.to_owned(),
        Err(err) => err.to_string(),
    }
}

#[get("/cowsay/<input>")]
fn cowsay(input: &str) -> String {
    let cowsay_output = Command::new("cowsay").arg(input).output();

    let response = match cowsay_output {
        Ok(out) => out.stdout,
        Err(err) => err.to_string().into(),
    };

    let response = std::str::from_utf8(&response);
    match response {
        Ok(res) => res.to_owned(),
        Err(err) => err.to_string(),
    }
}

#[get("/")]
fn index() -> String {
    cowsay("Hello World!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, cowsay, cowsay_animal])
}
