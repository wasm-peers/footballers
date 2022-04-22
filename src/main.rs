use footballers::components::FootballersApp;

fn main() {
    // TODO: make log level dynamic, for e.g. modifiable via a query parameter
    wasm_logger::init(wasm_logger::Config::new(log::Level::Error));
    yew::start_app::<FootballersApp>();
}
