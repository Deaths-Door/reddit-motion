use std::collections::HashMap;
use colored::Colorize;
use maplit::{convert_args, hashmap};
use unic_langid::LanguageIdentifier;
use fluent_templates::fluent_bundle::FluentValue;

fluent_templates::static_loader! {
    static LOCALES = {
        locales : "./assets/locales",
        fallback_language : "en",
    };
}

pub fn lookup_args<'a>(lang : &LanguageIdentifier,id : &str,map : &HashMap<&str,FluentValue<'a>>) -> String {
    LOCALES.lookup_single_language::<&str>(lang, id,Some(map))
        // Don't translate this
        .expect(&*"Specificed locale not availiable".bright_red())
}

pub fn lookup1(lang : &LanguageIdentifier,id : &str,arg_id : &str,value : &str) -> String {
    lookup_args(lang, id, &convert_args!(hashmap!(
        arg_id => value
    )))
}

pub fn lookup(lang : &LanguageIdentifier,id : &str) -> String {
    LOCALES.lookup_single_language::<&str>(lang, id,None)
        // Don't translate this
        .expect(&*"Specificed locale not availiable".bright_red())
}