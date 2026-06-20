use std::process;

fn main() {
    // TODO: wire up CLI arg parsing (clap) and iced app launch
    // For now just confirm the binary runs and GStreamer will init later.

    println!("velocity v{}", env!("CARGO_PKG_VERSION"));

    if let Err(e) = run() {
        eprintln!("fatal: {e}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // placeholder — this is where the iced Application::run() call will go
    // once we pull in the gui and engine crates.
    Ok(())
}
