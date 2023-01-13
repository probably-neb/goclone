#[macro_export]
macro_rules! iter_exclude {
    ($exclude: expr; $prefunc:expr => $func:expr) => {
        $exclude.iter().map($prefunc).flatten().map($func)
    };

    ($exclude:expr => $func:expr) => {
        iter_exclude!($exclude; std::convert::identity => $func)
    };

    ($exclude:expr) => {
        iter_exclude!($exclude => String::as_str)
    };

    ($exclude:expr; $prefunc:expr) => {
        iter_exclude!($exclude; $prefunc => String::as_str)
    };
}
