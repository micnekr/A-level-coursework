use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{groups, groups_participants, users};

use super::{events::ParticipationType, models::UnsavedModel, users::User};

#[derive(Identifiable, Queryable, Associations, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key = owner_id))]
#[diesel(table_name = groups)]
/// A struct that represents a group in the database
pub struct Group {
    pub id: i32,
    pub name: String,
    pub owner_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = groups)]
/// A group that has not been saved to the database yet
pub struct UnsavedGroup {
    pub name: String,
    pub owner_id: i32,
}

/// A struct that represents a user being in a group
#[derive(Identifiable, Queryable, Associations, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key = participant_id))]
#[diesel(belongs_to(Group, foreign_key = group_id))]
#[diesel(table_name = groups_participants)]
pub struct GroupParticipant {
    pub id: i32,
    pub group_id: i32,
    pub participant_id: i32,
    pub participation_type: ParticipationType,
}

#[derive(Insertable)]
#[diesel(table_name = groups_participants)]
/// A group that has not been saved to the database yet
pub struct UnsavedGroupParticipant {
    pub group_id: i32,
    pub participant_id: i32,
    pub participation_type: ParticipationType,
}

#[derive(Serialize)]
/// A struct that represents a user participating in an event
pub struct UserParticipationData {
    username: String,
    group_id: i32,
    user_id: i32,
}

impl Group {
    /// Gets the groups owned by a person
    pub fn get_owned_groups(connection: &mut PgConnection, user: &User) -> QueryResult<Vec<Group>> {
        groups::table
            .filter(groups::owner_id.eq(user.id))
            .load(connection)
    }

    /// Gets the participants of the groups the user owns
    pub fn get_owned_groups_participants(
        connection: &mut PgConnection,
        user: &User,
    ) -> Result<Vec<UserParticipationData>, diesel::result::Error> {
        // Load the users who participate in one of the events owned by the current user
        let groups_and_users: Vec<(String, i32, i32)> = groups::table
            .inner_join(groups_participants::table)
            .inner_join(users::table.on(users::id.eq(groups_participants::participant_id)))
            .filter(groups::owner_id.eq(user.id))
            // .filter(groups::owner_id.eq(user.id))
            .select((users::username, groups::id, users::id))
            .load(connection)?;

        // Represent the data in a better way
        Ok(groups_and_users
            .into_iter()
            .map(|(username, group_id, user_id)| UserParticipationData {
                username,
                group_id,
                user_id,
            })
            .collect())
    }

    /// Get the group by id
    pub fn get_group_by_id(
        connection: &mut PgConnection,
        group_id: i32,
    ) -> Result<Option<Self>, diesel::result::Error> {
        groups::table.find(group_id).first(connection).optional()
    }
}

impl UnsavedModel<Group> for UnsavedGroup {
    fn save(self, connection: &mut PgConnection) -> QueryResult<Group> {
        diesel::insert_into(groups::dsl::groups)
            .values(self)
            .get_result(connection)
    }
}

impl UnsavedModel<GroupParticipant> for UnsavedGroupParticipant {
    fn save(self, connection: &mut PgConnection) -> QueryResult<GroupParticipant> {
        diesel::insert_into(groups_participants::dsl::groups_participants)
            .values(self)
            .get_result(connection)
    }
}