use fluent_templates::{static_loader,Loader};
use unic_langid::LanguageIdentifier;
use lazy_static::lazy_static;

static_loader! {
    static LOCALES = {
        locales: r"./assets/locales",
        fallback_language: "en-US",
    };
}

lazy_static! {
    static ref CURRENT_LANGUAGE : LanguageIdentifier = .parse().unwrap();
}

pub fn println(id : &str) {
    println!("{}",&LOCALES.lookup(&CURRENT_LANGUAGE,id).unwrap());
}