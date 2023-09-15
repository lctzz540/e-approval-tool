mod process;
mod template;

fn main() {
    let filename = "template.json";

    match template::read_json_file(filename) {
        Some(_root) => {
            println!("loaded template")
        }
        None => {
            eprintln!("Error reading JSON file");
        }
    }
    match process::process_request("request.csv") {
        Ok(_) => {
            println!("OK");
        }
        Err(err) => {
            eprintln!("Error processing request: {:?}", err);
        }
    }
}
