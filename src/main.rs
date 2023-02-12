mod listener;
mod task;

fn main() {
    println!("{}: {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    listener::listen();
}
