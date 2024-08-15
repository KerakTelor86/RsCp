#[macro_export]
macro_rules! debug {
    ($body:block) => {
        #[cfg(feature = "local")]
        {
            $body;
        }
    };
}

#[macro_export]
macro_rules! debug_highlight {
    ($io:ident, $body:block) => {
        #[cfg(feature = "local")]
        {
            $io.write("\u{001b}[32m");
            $body;
            $io.write("\u{001b}[0m");
        }
    };
}

pub use debug;
pub use debug_highlight;