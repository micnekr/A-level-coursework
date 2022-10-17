use super::users::User;
use actix_session::{Session, SessionInsertError};

macro_rules! use_session {
    ($session: expr, $user_var: ident) => {
        use $crate::data::session::get_session;
        use $crate::endpoints::EndpointError;
        let $user_var = get_session($session);
        if $user_var.is_none() {
            return Result::Err(EndpointError::BadClientData(
                "Could not authenticate you. Please go to the login page.",
            ));
        }

        // Expose the user
        // Safety: we know it is not None
        let $user_var = $user_var.unwrap();
    };
}

// Export the macro
pub(crate) use use_session;

// Tries to get a user from a session
pub fn get_session(session: Session) -> Option<User> {
    if let (Ok(Some(id)), Ok(Some(username)), Ok(Some(password_hash))) = (
        session.get("id"),
        session.get("username"),
        session.get("password_hash"),
    ) {
        Some(User {
            id,
            username,
            password_hash,
        })
    } else {
        None
    }
}

/// Remembers the user for a session
/// Returns an Error if it can not serialise the user fields as JSON
pub fn set_session(session: Session, user: &User) -> Result<(), SessionInsertError> {
    session.insert("id", user.id)?;
    session.insert("username", &user.username)?;
    session.insert("password_hash", &user.password_hash)?;

    Ok(())
}
