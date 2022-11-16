use crate::page_template::ReactElement;

pub const PASSWORD_HASH_LENGTH: usize = 96;

pub const PORT: &str = "8080";
pub const DOMAIN: &str = "localhost";

pub const PROTOCOL: &str = "https";

/// Components that are always loaded on all pages
pub const COMPONENTS_ALWAYS_INCLUDED: &[ReactElement] = &[
    ReactElement::COMPONENT("App"),
    ReactElement::COMPONENT("Header"),
];
