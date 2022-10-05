use argon2::{Argon2, PasswordHasher};
pub use diesel::{connection, prelude::*};
use password_hash::SaltString;
use rand_core::OsRng;

use serde::Serialize;

use crate::schema::users;

use super::models::UnsavedModel;

#[derive(Insertable)]
#[diesel(table_name = users)]
/// A user that has not been saved to the database yet
pub struct UnsavedUser {
    pub username: String,
    pub password_hash: String,
}

#[derive(Queryable, Debug, Serialize)]
/// A user struct that represents a user record in a database
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

impl UnsavedUser {
    pub fn hash(text: &str) -> password_hash::Result<String> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let text = text.as_bytes();
        Ok(argon2.hash_password(text, &salt)?.to_string())
    }

    /// Create a new user, without performing any checks
    pub fn try_new(username: String, password: String) -> password_hash::Result<Self> {
        Ok(Self {
            username,
            password_hash: UnsavedUser::hash(&password)?,
        })
    }
}
impl UnsavedModel<User> for UnsavedUser {
    fn save(self, connection: &mut PgConnection) -> User {
        diesel::insert_into(users::dsl::users)
            .values(self)
            .get_result(connection)
            .expect("Error saving a new user")
    }
}
