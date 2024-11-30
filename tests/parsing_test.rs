#[cfg(test)]
#[path = "../src/parsing.rs"]
mod parsing;

mod tests {
    use super::*;
    use parsing::tools::parse_json_value;
    use std::time::Instant;
    use std::{env, fs, path::Path};

    #[test]
    fn gjson_parse_first() {
        let now;
        let elapsed;

        /*
        let path = Path::new("data/data.json");
        println!(
            "current path {:?} path {:?}",
            env::current_dir(),
            path.canonicalize()
        );*/
        let data: String = fs::read_to_string(Path::new("data/data.json")).expect(
            "Error loading JSON file",
        );

        now = Instant::now();
        let key = "closeTime";
        // There seems no way, we can tell the parser we want the second closeTime, not the first one, except parsing all into an array
        let parsed: gjson::Value = gjson::get(&data, "#(symbol=\"BTC-241206-115000-C\").closeTime");
        let value = parsed.str();
        assert_eq!(parsed.str(), "1732985135837");

        elapsed = now.elapsed();

        println!("gjson_parse_first: the value of {0:<10} {1:<16} {2:.2?}", key, value, elapsed);
    }

    #[test]
    fn own_parser_parse_first() {
        let now;
        let elapsed;

        /*
        let path = Path::new("data/data.json");
        println!(
            "current path {:?} path {:?}",
            env::current_dir(),
            path.canonicalize()
        );*/
        let data: String = fs::read_to_string(Path::new("data/data.json")).expect(
            "Error loading JSON file",
        );

        let key = "closeTime";
        let key_string = format!("\"{}\":", key);
        let key_bytes = key_string.as_bytes();
        let mut position = 50;
        now = Instant::now();
        match parse_json_value(data.as_ref(), key_bytes, &mut position) {
            Some(value) => {
                elapsed = now.elapsed();
                assert_eq!(value, "1732985135837");
                println!(
                    "own_parser_parse_first: the value of {0:<10}  {1:<16} pos {2:<10} {3:.2?}",
                    key, value, position, elapsed
                );
            }
            None => {
                println!("Key '{}' not found", key);
            }
        }
    }

    #[test]
    fn gjson_parse_all() {
        let now;
        let elapsed;

        /*
        let path = Path::new("data/data.json");
        println!(
            "current path {:?} path {:?}",
            env::current_dir(),
            path.canonicalize()
        );*/
        let data: String = fs::read_to_string(Path::new("data/data.json")).expect(
            "Error loading JSON file",
        );

        now = Instant::now();

        let key = "closeTime";
        // There seems no way, we can tell the parser we want the second closeTime, not the first one, except parsing all into an array
        let parsed: gjson::Value = gjson::get(&data, "#.closeTime");
        let value = parsed.array();
        assert_eq!(value[2].str(), "1732946532759");

        elapsed = now.elapsed();

        println!(
            "gjson_parse_all: The value of {0:<10} {1:<16} {2:.2?}",
            key,
            value[2].str(),
            elapsed
        );
    }

    #[test]
    fn own_parser_parse_all() {
        let now;
        let elapsed;
        /*
        let path = Path::new("data/data.json");
        println!(
            "current path {:?} path {:?}",
            env::current_dir(),
            path.canonicalize()
        );*/
        let data: String = fs::read_to_string(Path::new("data/data.json")).expect(
            "Error loading JSON file",
        );
        let key = "closeTime";
        let key_string = format!("\"{}\":", key);
        let key_bytes = key_string.as_bytes();
        let mut position = 0;
        let mut item_index = 0;
        let mut targte_index = 2;
        let mut result = Vec::new();
        now = Instant::now();
        loop {
            match parse_json_value(data.as_ref(), key_bytes, &mut position) {
                Some(value) => {
                    result.push(value);
                    item_index=item_index+1;
                    /*
                    // This optimization would speed up the item collection process, if not all items are needed
                    if item_index > targte_index + 1 {
                        break;
                    }
                    */
                }
                None => {
                    println!("Key '{}' not found", key);
                    break;
                }
            }
        }
        assert_eq!(result[2], "1732946532759");

        elapsed = now.elapsed();
        println!(
            "own_parser_parse_all: the value of {0:<10}  {1:<16} pos {2:<10} {3:.2?}",
            key, result[2], position, elapsed
        );
    }
}
