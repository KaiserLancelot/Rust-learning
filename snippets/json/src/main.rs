use mimalloc::MiMalloc;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() -> Result<()> {
    let data = br#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#
    .to_vec();

    let p: Person = serde_json::from_slice(&data)?;

    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}
