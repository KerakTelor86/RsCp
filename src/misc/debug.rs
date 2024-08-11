#[macro_export]
macro_rules! debug {
    (with_io($io:ident) $body:block) => {
        #[cfg(feature = "local")]
        {
            $io.write("\u{001b}[32m");
            $body;
            $io.write("\u{001b}[0m");
        }
    };
    (without_io() $body:block) => {
        #[cfg(feature = "local")]
        {
            $body;
        }
    };
}

pub use debug;
