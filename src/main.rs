use std::fs;
use rand::Rng;
use serde_json::{Value, json};

fn main() {
    // let mut random_number = rand::thread_rng().gen_range(1..=4);
    let res = fs::read_to_string("sounds.json");
    let s = match res {
        Ok(s) => s,
        Err(_) => panic!("UOOHHHHHHHHH FAILED TO REAWD sounds.json UUOOOOHHHHH 😭😭😭😭😭💢💢💢")
    };

    let mut json_data: serde_json::Value = serde_json::from_str(&s)
        .expect("UOOHHHHHHHHH FAILED TO PARSE JSON UUOOOOHHHHH 😭😭😭😭😭💢💢💢");

    fn process_sounds(sounds: &mut Value) {
        let mut rng = rand::thread_rng();
        if let Value::Array(arr) = sounds {
            for item in arr {
                if let Value::Object(obj) = item {
                    if let Some(name) = obj.get_mut("name") {
                        *name = json!(format!("mob/cat/meow{}", rng.gen_range(1..=4)));
                    }
                } else if let Value::String(_) = item {
                    *item = json!(format!("mob/cat/meow{}", rng.gen_range(1..=4)));
                }
            }
        }
    }

    for (_key, value) in json_data.as_object_mut().unwrap() {
        if let Some(sounds) = value.get_mut("sounds") {
            process_sounds(sounds);
        }
    }

    fs::write("output.json", serde_json::to_string_pretty(&json_data).unwrap())
        .expect("UOOHHHHHHHHH FAILED TO WRITE OUTPUTO JSON UUOOOOHHHHH 😭😭😭😭😭💢💢💢")
}
