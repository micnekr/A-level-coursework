use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{groups, groups_participants, users};

use super::{models::UnsavedModel, users::User};

#[derive(Serialize, Deserialize, PartialEq, Debug, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::ParticipationType"]
pub enum ParticipationType {
    Rejected,
    Accepted,
    NoResponse,
}

#[derive(Identifiable, Queryable, Associations, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key = owner_id))]
#[diesel(table_name = groups)]
/// A struct that represents a group in the database
pub struct Group {
    pub id: i32,
    pub is_special: bool,
    pub name: String,
    pub owner_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = groups)]
/// A group that has not been saved to the database yet
pub struct UnsavedGroup {
    pub is_special: bool,
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

    pub fn rename_group_by_id(
        connection: &mut PgConnection,
        group_id: i32,
        user: &User,
        new_name: String,
    ) -> QueryResult<Group> {
        diesel::update(groups::table)
            .filter(
                // Only allow the owner edit the group
                groups::owner_id.eq(user.id).and(groups::id.eq(group_id)),
            )
            .set(groups::name.eq(new_name))
            .get_result(connection)
    }

    /// A function to remove a user from a group. Does not consider permissions
    pub fn remove_user(
        connection: &mut PgConnection,
        group: &Group,
        user_id: i32,
    ) -> QueryResult<usize> {
        diesel::delete(groups_participants::table)
            .filter(
                // Remove an entry with the correct group and user
                groups_participants::group_id
                    .eq(group.id)
                    .and(groups_participants::participant_id.eq(user_id)),
            )
            .execute(connection)
    }

    /// A function that accepts or rejects an event for a user
    pub fn reply_to_group_invitation(
        connection: &mut PgConnection,
        group_id: i32,
        user: &User,
        decision: ParticipationType,
    ) -> QueryResult<usize> {
        diesel::update(groups_participants::table)
            .filter(
                groups_participants::group_id
                    .eq(group_id)
                    .and(groups_participants::participant_id.eq(user.id)),
            )
            .set(groups_participants::participation_type.eq(decision))
            .execute(connection)
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
