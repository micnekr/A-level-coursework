use crate::schema::{friendships, users};
use diesel::dsl::count;
pub use diesel::{connection, prelude::*};
use serde::{Deserialize, Serialize};

use super::{models::UnsavedModel, users::User};

/// A struct that represents a friend relation between 2 users. It is unidirectional, with the
/// "owner" being the person who initiated the friendship
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key = owner_id))]
#[diesel(table_name = friendships)]
pub struct Friendship {
    pub id: i32,
    owner_id: i32,
    friend_id: i32,
}

/// A struct for a friendship that can be saved in the database
#[derive(Insertable)]
#[diesel(table_name = friendships)]
pub struct UnsavedFrienship {
    owner_id: i32,
    friend_id: i32,
}

/// An enum used to designate the result of adding a friend, whether it was successful and why it
/// failed
#[derive(PartialEq, Eq)]
pub enum FriendAddResult {
    UsernameNotFound,
    TriedFriendThemselves,
    AlreadyFriends,
    Success,
}

impl Friendship {
    /// A function to get all friends of a user (unidirectional)
    pub fn get_friends(
        connection: &mut PgConnection,
        user: &User,
    ) -> Result<Vec<User>, diesel::result::Error> {
        // Query all the friends where the user is the "owner" of the friendship
        let friends_vec = users::table
            .inner_join(friendships::table.on(friendships::friend_id.eq(users::id)))
            .filter(friendships::owner_id.eq(user.id))
            .select(users::all_columns)
            .load::<User>(connection)?;
        Ok(friends_vec)
    }

    /// A function to add a friend (unidirectional)
    pub fn add_friend(
        connection: &mut PgConnection,
        friendship_owner: &User,
        friend_username: &String,
    ) -> Result<FriendAddResult, diesel::result::Error> {
        // Check that we are not trying to friend ourselves
        if friendship_owner.username == *friend_username {
            return Ok(FriendAddResult::TriedFriendThemselves);
        }
        // Get the user by the username
        // A vec of 0 or 1 items
        let mut found_users: Vec<_> = users::table
            .filter(
                // Search for this specific user
                users::username.eq(friend_username),
            )
            .limit(1)
            .select(users::all_columns)
            .load::<User>(connection)?;

        let friend = found_users.pop();

        if let Some(friend) = friend {
            // Check if the friendship exists
            // No need to worry about concurrent access, since the connection is protected by a mutex.
            let number_of_existing_friendships: i64 = friendships::table
                .filter(
                    friendships::owner_id
                        .eq(friendship_owner.id)
                        .and(friendships::friend_id.eq(friend.id)),
                )
                .select(count(friendships::id))
                .first(connection)?;
            if number_of_existing_friendships != 0 {
                return Ok(FriendAddResult::AlreadyFriends);
            }

            let friendship = UnsavedFrienship {
                owner_id: friendship_owner.id,
                friend_id: friend.id,
            };
            friendship.save(connection)?;

            Ok(FriendAddResult::Success)
        } else {
            Ok(FriendAddResult::UsernameNotFound)
        }
    }
}

impl UnsavedModel<Friendship> for UnsavedFrienship {
    fn save(self, connection: &mut PgConnection) -> QueryResult<Friendship> {
        diesel::insert_into(friendships::dsl::friendships)
            .values(self)
            .get_result(connection)
    }
}
