mod parsing;
use parsing::tools::parse_json_value;


fn make_request() -> Result<String, Box<dyn std::error::Error>> {
    /*
    use std::{fs::File, io::Write};
    // Send the GET request to the URL
    let response = minreq::get("https://eapi.binance.com/eapi/v1/ticker").send()?;

    // Check if the response status code is 200 (OK)
    if response.status_code != 200 {
        return Err(format!("Error: Received status code {}", response.status_code).into());
    }

    // Get the body of the response as a String
    let body: String = String::from_utf8_lossy(response.as_bytes()).to_string(); // this extracts the body as a string
    let filename = "data.json";
    let mut file = File::create(filename).unwrap();
    file.write_all(body.as_bytes()).unwrap();

    Ok(body)
    */
    
    use std::fs;
    /*
    use std::env;
    use std::path::Path;
    
    let path = Path::new("data/data.json");
    println!(
        "current path {:?} path {:?}",
        env::current_dir(),
        path.canonicalize()
    );
    */
    // Read file into a string
    let content = fs::read_to_string("data/data.json")?;

    Ok(content)
}

fn main() {
    use std::time::Instant;
    let mut now;
    let mut elapsed;
    match make_request() {
        Ok(response) => {
            //println!("{}", response);
            let mut position: usize = 0;
            let key = "closeTime";

            let key_string = format!("\"{}\":", key);
            let key_bytes = key_string.as_bytes();

            loop {
                now = Instant::now();
                match parse_json_value(response.as_ref(), key_bytes, &mut position) {
                    Some(value) => {
                        elapsed = now.elapsed();

                        println!(
                            "The value of {0:<10}  {1:<16} pos {2:<10} {3:.2?}",
                            key, value, position, elapsed
                        );
                    }
                    None => {
                        println!("Key '{}' not found", key);
                        break;
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Request failed: {}", err);
        }
    }
}
