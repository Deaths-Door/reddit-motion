use anyhow::Context;
use colored::Colorize;
use unic_langid::LanguageIdentifier;


// TODO: Translate all into all locales
fluent_templates::static_loader! {
    static LOCALES = {
        locales : "./assets/locales",
        fallback_language : "en-US",
    };
}


pub fn lookup(lang : &LanguageIdentifier,id : &str) -> anyhow::Result<String> {
    LOCALES.lookup_single_language::<&str>(lang, id,None)
        // Don't translate this
        .context("Specificed locale not availiable".bright_red())
}