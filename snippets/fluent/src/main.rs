use fluent::{FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

fn main() {
    let ftl_string = String::from("hello-world = Hello, world!");
    let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

    let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
    let mut bundle = FluentBundle::new(vec![langid_en]);

    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");

    let msg = bundle
        .get_message("hello-world")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, None, &mut errors);

    assert_eq!(&value, "Hello, world!");
}
