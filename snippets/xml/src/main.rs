use quick_xml::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Lang {
    En,
    Fr,
    De,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Html {
    lang: Option<String>,
}

fn main() {
    let xml = "
        <!DOCTYPE html>
        <html lang=\"en\">
        </html>
}";
    let _html: Html = from_str(xml).expect("Error");
}
