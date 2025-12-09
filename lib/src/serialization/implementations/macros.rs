#[macro_export]
macro_rules! hr_serializable {
    ($ty:ty) => {
        impl $crate::serialization::readable::Readable for $ty {
            fn from_reader<R: std::io::Read>(reader: &mut R) -> anyhow::Result<Self> {
                let res = $crate::serialization::serializers::hr_serializer::from_reader(reader);
                <anyhow::Result<Self>>::context(
                    res,
                    format!("Failed to deserialize {}", stringify!($ty)),
                )
            }
        }

        impl $crate::serialization::writable::Writable for $ty {
            fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()> {
                let res =
                    $crate::serialization::serializers::hr_serializer::to_writer(writer, self);
                <anyhow::Result<()>>::context(
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
        impl $crate::serialization::readable::Readable for $ty {
            fn from_reader<R: std::io::Read>(reader: &mut R) -> anyhow::Result<Self> {
                let res = $crate::serialization::serializers::bin_serializer::from_reader(reader);
                <anyhow::Result<Self>>::context(
                    res,
                    format!("Failed to deserialize {}", stringify!($ty)),
                )
            }
        }

        impl $crate::serialization::writable::Writable for $ty {
            fn to_writer<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()> {
                let res =
                    $crate::serialization::serializers::bin_serializer::to_writer(writer, self);
                <anyhow::Result<()>>::context(
                    res,
                    format!("Failed to serialize {}", stringify!($ty)),
                )
            }
        }
    };
}
