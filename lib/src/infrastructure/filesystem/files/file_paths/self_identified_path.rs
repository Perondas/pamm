use crate::domain::pack::pack_config::PackConfig;
use crate::infrastructure::filesystem::files::file_paths::fixed_path::FixedFilePath;
use crate::infrastructure::filesystem::files::file_paths::rel_path::RelPath;

pub(crate) trait SelfIdentifiedFilePath {
    fn file_path(&self) -> RelPath;
}

#[macro_export]
macro_rules! at_pack_root {
    ($ty:ty) => {
        impl $crate::infrastructure::filesystem::files::file_paths::self_identified_path::SelfIdentifiedFilePath for $ty {
            fn file_path(&self) -> RelPath {
                static_assertions::assert_impl_all!($ty: $crate::infrastructure::filesystem::files::file_names::fixed_file::FixedFile, $crate::domain::self_keyed::SelfKeyed);
                RelPath::from_name(<Self as $crate::domain::self_keyed::SelfKeyed>::get_key(self))
            }
        }
    };
}

at_pack_root!(PackConfig);

impl<T> SelfIdentifiedFilePath for T
where
    T: FixedFilePath,
{
    fn file_path(&self) -> RelPath {
        T::file_path()
    }
}
