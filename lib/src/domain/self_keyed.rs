pub trait SelfKeyed {
    fn get_key(&self) -> &str;
}

#[macro_export]
macro_rules! keyed {
    ($ty:ty) => {
        impl $crate::domain::self_keyed::SelfKeyed for $ty {
            fn get_key(&self) -> &str {
                &self.name
            }
        }
    };
}
