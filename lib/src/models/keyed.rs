pub trait Keyed {
    fn get_key(&self) -> &str;
}

#[macro_export]
macro_rules! named {
    ($ty:ty) => {
        impl $crate::models::keyed::Keyed for $ty {
            fn get_key(&self) -> &str {
                &self.name
            }
        }
    };
}
