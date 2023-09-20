#![allow(clippy::no_effect_underscore_binding)]

use std::process::Command;

#[macro_use]
extern crate rocket;

const COWSAY_PATH: &str = "/usr/games/cowsay";

#[get("/<input>/<kind>")]
fn cowsay_animal(input: &str, kind: &str) -> String {
    let cowsay_output = Command::new(COWSAY_PATH).args(["-f", kind, input]).output();

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

#[get("/<input>")]
fn cowsay(input: &str) -> String {
    let cowsay_output = Command::new(COWSAY_PATH).arg(input).output();

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

#[get("/kinds")]
fn kinds() -> String {
    match Command::new(COWSAY_PATH).arg("-l").output() {
        Ok(out) => match std::str::from_utf8(&out.stdout) {
            Ok(conv) => cowsay(conv),
            Err(err) => err.to_string(),
        },
        Err(err) => err.to_string(),
    }
}

#[get("/help")]
fn help() -> String {
    cowsay(
        r#"/<input>/ -> cowsay <input>
        /<input>/<kind>/ -> cowsay -f <kind> <input>
        /help/ -> list these endpoints
        /kinds/ -> cowsay -l"#,
    )
}

#[get("/")]
fn index() -> String {
    cowsay("Hello World!\n\ngoto /help/ for help\n\ngoto /kinds/ to show kinds")
}

#[launch]
fn rocket() -> _ {
    let routes = routes![index, cowsay, cowsay_animal, help, kinds];
    rocket::build().mount("/", routes)
}
