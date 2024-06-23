use super::Resolver;
pub use crate::infrastructure_layer::data::control_type::DefaultValue;

impl Resolver<DefaultValue> {
    pub const I32_DEFAULT_VALUE: i32 = i32::MIN;
    pub const STRING_DEFAULT_VALUE: String = String::new();
    pub const STRING_MINIMUM_LENGTH_VALUE: String = String::new();

    pub fn to_option<T>(subject: T) -> Option<T>
    where
        T: Default + Eq,
    {
        let result = if subject != <T as Default>::default() {
            Some(subject)
        } else {
            None
        };

        return result;
    }

    pub fn from_option<T>(subject: Option<T>) -> T
    where
        T: Default,
    {
        let result = match subject {
            Some(subject_) => subject_,
            None => <T as Default>::default(),
        };

        return result;
    }
}
