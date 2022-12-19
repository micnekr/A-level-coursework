use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{groups, groups_participants};

use super::users::User;

#[derive(Identifiable, Queryable, Associations, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key = owner_id))]
#[diesel(table_name = groups)]
/// A struct that represents a group in the database
pub struct Group {
    pub id: i32,
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
}

impl Group {
    /// Gets all groups a user is an owner of
    pub fn get_owned_groups(
        connection: &mut PgConnection,
        user: &User,
    ) -> Result<Vec<Group>, diesel::result::Error> {
        groups::table
            .filter(groups::owner_id.eq(user.id))
            .select(groups::all_columns)
            .load::<Group>(connection)
    }

    /// Get the group by id
    pub fn get_group_by_id(
        connection: &mut PgConnection,
        group_id: i32,
    ) -> Result<Option<Self>, diesel::result::Error> {
        groups::table.find(group_id).first(connection).optional()
    }
}
