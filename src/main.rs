use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if an argument was provided
    if args.len() < 2 {
        eprintln!("Usage: {} <header_name>", args[0]);
        return;
    }

    // Extract section_name from command line argument
    let header_name = &args[1];

    let text = format!(r#"/*//////////////////////////////////////////////////////////////
                            {}
//////////////////////////////////////////////////////////////*/"#, header_name);
    println!("{}", text);
}
