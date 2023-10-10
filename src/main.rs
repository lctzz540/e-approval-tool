use clap::Parser;
mod make_workflow;
mod process;
mod template;
mod utils;
mod workflow;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    mode: String,
}

fn main() {
    let args = Args::parse();
    if args.mode == "template" {
        match process::process_request("request.csv") {
            Ok(_) => {
                println!("Done! please check output.json");
            }
            Err(err) => {
                eprintln!("Error processing request: {:?}", err);
            }
        }
    } else if args.mode == "workflow" {
        match make_workflow::make_workflow() {
            Ok(_) => {
                println!("Done!")
            }
            Err(err) => {
                eprintln!("Error making workflow: {:?}", err)
            }
        }
    }
}
