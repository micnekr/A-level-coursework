use super::{
    events::{Event, ParticipationType},
    users::User,
};
use diesel::BoolExpressionMethods;
use diesel::RunQueryDsl;

use crate::schema::events;
use crate::schema::events_participants;
use diesel::ExpressionMethods;
use diesel::{PgConnection, QueryDsl};
use serde::Serialize;

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
        // Fetch all the events which have no response to display a corresponding notification
        let events_without_response = events::table
            .inner_join(events_participants::table)
            .filter(
                // Has to be a participant and should have accepted the invitation
                events_participants::participant_id
                    .eq(user.id)
                    .and(events_participants::participation_type.eq(ParticipationType::NoResponse)),
            )
            .select(events::all_columns)
            .load::<Event>(connection)?;

        // express the events as notifications
        let notifications: Vec<_> = events_without_response
            .into_iter()
            .map(|event| Notification::Invitation(event))
            .collect();

        Ok(notifications)
    }
}
