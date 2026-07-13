pub fn init() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .parse_default_env()
        .init();
}
