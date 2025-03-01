use std::io::Write;

pub fn init_logging() {
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            writeln!(buf, "{} - {}", record.level(), record.args())
        })
        .init()
}
