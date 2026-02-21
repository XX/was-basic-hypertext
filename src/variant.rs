use strum::{AsRefStr, IntoStaticStr};

#[derive(Copy, Clone, Debug, Default, IntoStaticStr, AsRefStr)]
#[strum(const_into_str, serialize_all = "snake_case")]
pub enum Variant {
    #[default]
    Neutral,
    Brand,
    Success,
    Warning,
    Danger,
}

impl Variant {
    pub const NEUTRAL: &str = Variant::Neutral.into_str();
    pub const BRAND: &str = Variant::Brand.into_str();
    pub const SUCCESS: &str = Variant::Success.into_str();
    pub const WARNING: &str = Variant::Warning.into_str();
    pub const DANGER: &str = Variant::Danger.into_str();
}
