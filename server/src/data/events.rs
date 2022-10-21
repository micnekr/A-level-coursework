pub use diesel::{connection, prelude::*};
use serde::{Deserialize, Serialize};

use crate::schema::{events, events_participants};

use super::{models::UnsavedModel, users::User};

#[derive(Serialize, Deserialize, PartialEq, Debug, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::RecurrenceType"]
pub enum RecurrenceType {
    Weekly,
    Once,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::VisibilityType"]
pub enum VisibilityType {
    Public,
    Private,
}

#[derive(Insertable)]
#[diesel(belongs_to(User, foreign_key = owner_id))]
#[diesel(table_name = events)]
/// An event that has not been saved to the database yet
pub struct UnsavedEvent {
    pub owner_id: i32,
    pub title: String,
    pub visibility: VisibilityType,
    pub start_time: i32,
    pub duration: i32,
    pub recurrence_type: RecurrenceType,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key = owner_id))]
#[diesel(table_name = events)]
/// An event struct that represents an event record in a database
pub struct Event {
    pub id: i32,
    pub owner_id: i32,
    pub title: String,
    pub visibility: VisibilityType,
    pub start_time: i32,
    pub duration: i32,
    pub recurrence_type: RecurrenceType,
}

/// A struct that represents a linking table for events and their participants
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(Event, foreign_key = event_id))]
#[diesel(belongs_to(User, foreign_key = participant_id))]
#[diesel(table_name = events_participants)]
pub struct EventParticipant {
    pub id: i32,
    event_id: i32,
    participant_id: i32,
}

impl Event {
    /// A function that gets all events that a user needs to be aware of
    pub fn get_events_with_user(
        connection: &mut PgConnection,
        user: &User,
    ) -> Result<Vec<Event>, diesel::result::Error> {
        // Query the events where the user is a participant or owner
        let events = events::table
            .inner_join(events_participants::table)
            .filter(
                events_participants::participant_id
                    .eq(user.id)
                    .or(events::owner_id.eq(user.id)),
            )
            .select(events::all_columns)
            .load::<Event>(connection)?;

        Ok(events)
    }
}

impl UnsavedModel<Event> for UnsavedEvent {
    fn save(self, connection: &mut PgConnection) -> QueryResult<Event> {
        diesel::insert_into(events::dsl::events)
            .values(self)
            .get_result(connection)
    }
}
