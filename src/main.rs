#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use std::fs::File;
use std::io;
use std::io::Read;
use rocket::http::{Cookie, Cookies};
use rocket::response::{content, Redirect};

#[get("/")]
fn list(mut cookies:Cookies) -> content::Html<String> {
    let k = cookies.get("YOU");
    match k {
        Some(T) => (),
        None => cookies.add(Cookie::new("YOU","USER"))
    }
    return ReadHTMLFile("src/html/list.html");
}

#[get("/alert/<id>")]
fn alert(id:String) -> content::Html<String> {
    return ReadHTMLFile("src/html/alert1.html");
}

#[get("/re/<page>")]
fn redirect(page:String,mut cookies:Cookies) -> Redirect {
    let page :i32 = page.parse().unwrap();
    match page {
        1=>Redirect::to("/alert/1"),
        2=>Redirect::to("/read/2"),
        3=>Redirect::to("/read/3"),
        _=>Redirect::to("/")
    }
}

#[get("/read/<page>")]
fn page(cookies:Cookies,page:String) -> content::Html<String>
{
    let page :i32 = page.parse().unwrap();
    if page == 666
    {
        if cookies.get("YOU").unwrap().value() == "ADMIN"
        {
            return ReadHTMLFile("src/html/666.html");
        }
        else
        {
            return ReadHTMLFile("src/html/alert2.html");
        }

    }
    return ReadHTMLFile(format!("src/html/{}.html",page).as_str());
}

fn ReadHTMLFile(path:&str) -> content::Html<String>
{
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    return content::Html(contents);
}

fn main()
{
    rocket::ignite().mount("/",routes![list,alert,redirect,page]).launch();
}