use std::collections::HashMap;
use std::io;

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut line = String::new();

        let size = io::stdin()
            .read_line(&mut line)
            .expect("read_line() failed");
        // EOF
        if size == 0 {
            break;
        }

        add_employee(&line, &mut map);
    }

    for value in map.values_mut() {
        value.sort_unstable();
    }

    for (key, value) in &map {
        println!("{}: {:?}", key, value);
    }
}

fn add_employee(str: &str, map: &mut HashMap<String, Vec<String>>) {
    let (employee, department) = process_command(str);

    let v = map.entry(department).or_insert(Vec::new());
    v.push(employee);
}

fn process_command(str: &str) -> (String, String) {
    let words: Vec<&str> = str.split_whitespace().collect();

    assert_eq!(words.len(), 4);
    assert_eq!(words[0], "Add");
    assert_eq!(words[2], "to");

    (words[1].to_string(), words[3].to_string())
}
