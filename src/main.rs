fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    corgi::cli::run()
}
