use diesel::PgConnection;
use serde::Serialize;

use super::{events::Event, users::User};

/// A list of `Notification`s
#[derive(Serialize)]
pub struct NotificaitonVec(pub Vec<Notification>);

/// An enum representing a notification to be shown to a user
#[derive(Serialize)]
pub enum Notification {
    Invitation(Event),
}

impl Notification {
    pub fn get_user_notifications(
        connection: &mut PgConnection,
        user: &User,
    ) -> Result<Vec<Notification>, diesel::result::Error> {
        Ok(vec![])
    }
}
