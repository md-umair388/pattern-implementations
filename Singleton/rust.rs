// Rust Singleton Implementation Example
use std::sync::{OnceLock, Once};

struct Singleton {
    // fields here
}

static INSTANCE: OnceLock<Singleton> = OnceLock::new();

impl Singleton {
    fn get_instance() -> &'static Singleton {
        INSTANCE.get_or_init(|| Singleton { })
    }
}