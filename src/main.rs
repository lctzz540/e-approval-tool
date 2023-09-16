mod process;
mod template;

fn main() {
    match process::process_request("request.csv") {
        Ok(_) => {
            println!("OK");
        }
        Err(err) => {
            eprintln!("Error processing request: {:?}", err);
        }
    }
}
