use icu_locid::{subtags, LanguageIdentifier};
use std::env;

const DEFAULT_INPUT: &str =
    "de, en-us, zh-hant, sr-cyrl, fr-ca, es-cl, pl, en-latn-us, ca-valencia, und-arab";

fn filter_input(input: &str) -> String {
    let langids = input.split(',').filter_map(|s| s.trim().parse().ok());

    let en_lang: subtags::Language = "en".parse().expect("Failed to parse language subtag.");

    let en_langids = langids.filter(|langid: &LanguageIdentifier| langid.language == en_lang);

    let en_strs: Vec<String> = en_langids.map(|langid| langid.to_string()).collect();

    en_strs.join(", ")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if let Some(input) = args.get(1) {
        input.as_str()
    } else {
        DEFAULT_INPUT
    };
    let _output = filter_input(input);

    println!("Input: {}\nOutput: {}", input, _output);
}
