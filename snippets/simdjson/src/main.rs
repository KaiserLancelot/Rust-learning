use mimalloc::MiMalloc;
use serde::{Deserialize, Serialize};
use simd_json::Result;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() -> Result<()> {
    let mut data = br#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#
    .to_vec();

    let p: Person = simd_json::serde::from_slice(&mut data)?;

    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}
