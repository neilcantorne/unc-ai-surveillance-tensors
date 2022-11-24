
#[derive(Clone, Debug)]
pub struct Identifier(String);

impl From<String> for Identifier {
    #[inline]
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Identifier {
    #[inline]
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}