pub trait Identifiable {
    fn get_identifier(&self) -> &str;
}

#[macro_export]
macro_rules! named {
    ($ty:ty) => {
        impl $crate::identifiable::Identifiable for $ty {
            fn get_identifier(&self) -> &str {
                &self.name
            }
        }
    };
}
