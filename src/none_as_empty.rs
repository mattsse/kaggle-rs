use serde::{Serialize, Serializer};

/// Serialize a string from `Option<T>` using `AsRef<str>` or using the empty
/// string if `None`.
pub fn serialize<T, S>(option: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    if let Some(value) = option {
        value.serialize(serializer)
    } else {
        "".serialize(serializer)
    }
}
