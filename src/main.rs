pub mod log;

fn main() {
    debug!("main", "Hello, world!");
    error!("main", "Something is wrong ({})!", 1);
}
