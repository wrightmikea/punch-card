// IBM 1130 Punch Card Simulator - CLI Server
//
// Command-line tool to serve the Yew web application

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "punch-card")]
#[command(about = "IBM 1130 Punch Card Simulator - Serves the web application", long_about = None)]
struct Args {
    /// Port to serve the application on
    #[arg(short, long, default_value_t = 9267)]
    port: u16,
}

fn main() {
    let args = Args::parse();

    println!("IBM 1130 Punch Card Simulator");
    println!("Serving on port: {}", args.port);
    println!("Coming soon: HTTP server implementation");

    // TODO: Implement warp/actix-web server
    // TODO: Serve static WASM bundle from crates/web/dist
}
