use std::collections::HashMap;
use anyhow::Context;
use colored::Colorize;
use maplit::{convert_args, hashmap};
use unic_langid::LanguageIdentifier;
use fluent_templates::fluent_bundle::FluentValue;

// TODO: Translate all into all locales
fluent_templates::static_loader! {
    static LOCALES = {
        locales : "./assets/locales",
        fallback_language : "en-US",
    };
}

pub fn lookup_args<'a>(lang : &LanguageIdentifier,id : &str,map : &HashMap<&str,FluentValue<'a>>) -> anyhow::Result<String> {
    LOCALES.lookup_single_language::<&str>(lang, id,Some(map))
        // Don't translate this
        .context("Specificed locale not availiable".bright_red())
}

pub fn lookup1<'a>(lang : &LanguageIdentifier,id : &str,arg_id : &str,value : &str) -> anyhow::Result<String> {
    lookup_args(lang, id, &convert_args!(hashmap!(
        arg_id => value
    )))
}

pub fn lookup(lang : &LanguageIdentifier,id : &str) -> anyhow::Result<String> {
    LOCALES.lookup_single_language::<&str>(lang, id,None)
        // Don't translate this
        .context("Specificed locale not availiable".bright_red())
}