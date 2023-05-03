use crate::page_template::ReactElement;

/// The number of characters in the password hash
pub const PASSWORD_HASH_LENGTH: usize = 96;

/// The port on which the server is listening
pub const PORT: &str = "8080";
/// The address of the server itself
pub const DOMAIN: &str = "localhost";

/// The protocol that is used for web requests; is HTTP or HTTPS
pub const PROTOCOL: &str = "https";

/// Components that are always loaded on all pages
pub const COMPONENTS_ALWAYS_INCLUDED: &[ReactElement] = &[
    ReactElement::COMPONENT("App"),
    ReactElement::COMPONENT("Header"),
];
