pub trait KnownName {
    fn known_name() -> &'static str;
}

#[macro_export]
macro_rules! known_name {
    ($ty:ty, $expr:expr) => {
        impl $crate::known_name::KnownName for $ty {
            fn known_name() -> &'static str {
                $expr
            }
        }
    };
}
