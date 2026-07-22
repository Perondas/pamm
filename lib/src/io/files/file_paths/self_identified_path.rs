use crate::io::files::file_paths::rel_path::RelPath;
use crate::models::pack::pack_config::PackConfig;

pub(crate) trait SelfIdentifiedFilePath {
    fn file_path(&self) -> RelPath;
}

#[macro_export]
macro_rules! at_pack_root {
    ($ty:ty) => {
        impl $crate::io::files::file_paths::self_identified_path::SelfIdentifiedFilePath for $ty {
            fn file_path(&self) -> RelPath {
                static_assertions::assert_impl_all!($ty: $crate::io::files::file_names::fixed_file::FixedFile, $crate::models::keyed::Keyed);
                RelPath::from_name(<Self as $crate::models::self_keyed::SelfKeyed>::get_key(self))
            }
        }
    };
}

at_pack_root!(PackConfig);
