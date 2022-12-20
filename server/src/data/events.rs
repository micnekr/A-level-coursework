pub use diesel::{connection, prelude::*};
use serde::{Deserialize, Serialize};

use crate::schema::{events, groups, groups_participants};

use super::{
    group::{Group, ParticipationType},
    models::UnsavedModel,
    users::User,
};

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
#[diesel(belongs_to(Group, foreign_key = group_id))]
#[diesel(table_name = events)]
/// An event that has not been saved to the database yet
pub struct UnsavedEvent {
    pub title: String,
    pub visibility: VisibilityType,
    pub start_time: i32,
    pub duration: i32,
    pub recurrence: RecurrenceType,
    pub group_id: i32,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(Group, foreign_key = group_id))]
#[diesel(table_name = events)]
/// An event struct that represents an event record in a database
pub struct Event {
    pub id: i32,
    pub title: String,
    pub visibility: VisibilityType,
    pub start_time: i32,
    pub duration: i32,
    pub recurrence_type: RecurrenceType,
    pub group_id: i32,
}

impl Event {
    /// get all events that were created by a group that the user is an admin of
    pub fn get_events_owned_by_user(
        connection: &mut PgConnection,
        user: &User,
    ) -> QueryResult<Vec<Event>> {
        groups::table
            .inner_join(events::table)
            .filter(groups::owner_id.eq(user.id))
            .select(events::all_columns)
            .load::<Event>(connection)
    }

    /// get all events that the user participates in, without being an owner
    pub fn get_accepted_events_participated_in_by_user(
        connection: &mut PgConnection,
        user: &User,
    ) -> QueryResult<Vec<Event>> {
        groups::table
            .inner_join(groups_participants::table)
            .inner_join(events::table)
            .filter(
                groups_participants::participant_id
                    .eq(user.id)
                    .and(groups_participants::participation_type.eq(ParticipationType::Accepted)),
            )
            .select(events::all_columns)
            .load::<Event>(connection)
    }

    /// A function that gets all events that a user needs to be aware of
    pub fn get_accepted_events_with_user(
        connection: &mut PgConnection,
        user: &User,
    ) -> Result<Vec<Event>, diesel::result::Error> {
        // Query the events where the user is an owner
        let events_as_owner = Event::get_events_owned_by_user(connection, user)?;

        // Query all the events where the user is a participant
        let mut events_as_participant =
            Event::get_accepted_events_participated_in_by_user(connection, user)?;
        // Merge the two vectors
        events_as_participant.extend(events_as_owner);

        Ok(events_as_participant)
    }
}

impl UnsavedModel<Event> for UnsavedEvent {
    fn save(self, connection: &mut PgConnection) -> QueryResult<Event> {
        diesel::insert_into(events::dsl::events)
            .values(self)
            .get_result(connection)
    }
}
