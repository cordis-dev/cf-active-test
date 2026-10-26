#[macro_export]
macro_rules! default_port {
    () => {
        crate::crate_in_macro_def::port()
    };
}

pub fn port() -> u16 {
    8080
}
