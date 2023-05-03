use super::group::ParticipationType;
use super::{group::Group, users::User};
use diesel::BoolExpressionMethods;
use diesel::RunQueryDsl;

use crate::schema::{groups, groups_participants};
use diesel::ExpressionMethods;
use diesel::{PgConnection, QueryDsl};
use serde::Serialize;

/// A list of `Notification`s
#[derive(Serialize)]
pub struct NotificaitonVec(pub Vec<Notification>);

/// An enum representing a notification to be shown to a user
#[derive(Serialize)]
pub enum Notification {
    Invitation(Group),
}

impl Notification {
    /// Get all notifications that belong to a user
    pub fn get_user_notifications(
        connection: &mut PgConnection,
        user: &User,
    ) -> Result<Vec<Notification>, diesel::result::Error> {
        // Fetch all the events which have no response to display a corresponding notification
        let events_without_response = groups::table
            .inner_join(groups_participants::table)
            .filter(
                // Has to be a participant and should have accepted the invitation
                groups_participants::participant_id
                    .eq(user.id)
                    .and(groups_participants::participation_type.eq(ParticipationType::NoResponse)),
            )
            // get the result from the database
            .select(groups::all_columns)
            .load::<Group>(connection)?;

        // express the events as notifications
        let notifications: Vec<_> = events_without_response
            .into_iter()
            .map(|event| Notification::Invitation(event))
            .collect();

        Ok(notifications)
    }
}
