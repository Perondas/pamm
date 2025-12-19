#[macro_export]
macro_rules! hr_serializable {
    ($ty:ty) => {
        impl $crate::io::serialization::readable::Readable for $ty {
            fn from_reader<R: std::io::Read>(reader: &mut R) -> anyhow::Result<Self> {
                let res =
                    $crate::io::serialization::serializers::hr_serializer::from_reader(reader);
                <anyhow::Result<Self> as anyhow::Context<_, _>>::context(
                    res,
                    format!("Failed to deserialize {}", stringify!($ty)),
                )
            }
        }

        impl $crate::io::serialization::writable::Writable for $ty {
            fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()> {
                let res =
                    $crate::io::serialization::serializers::hr_serializer::to_writer(writer, self);
                <anyhow::Result<()> as anyhow::Context<_, _>>::context(
                    res,
                    format!("Failed to serialize {}", stringify!($ty)),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! bin_serializable {
    ($ty:ty) => {
        impl $crate::io::serialization::readable::Readable for $ty {
            fn from_reader<R: std::io::Read>(reader: &mut R) -> anyhow::Result<Self> {
                let res =
                    $crate::io::serialization::serializers::bin_serializer::from_reader(reader);
                <anyhow::Result<Self> as anyhow::Context<_, _>>::context(
                    res,
                    format!("Failed to deserialize {}", stringify!($ty)),
                )
            }
        }

        impl $crate::io::serialization::writable::Writable for $ty {
            fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()> {
                let res =
                    $crate::io::serialization::serializers::bin_serializer::to_writer(writer, self);
                <anyhow::Result<()> as anyhow::Context<_, _>>::context(
                    res,
                    format!("Failed to serialize {}", stringify!($ty)),
                )
            }
        }
    };
}
