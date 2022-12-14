use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
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

/// A version of the user struct that can be sent to the frontend, i.e. it does not disclose
/// sensitive information
#[derive(Serialize)]
pub struct UserPublic {
    pub username: String,
    pub id: i32,
}

#[derive(Identifiable, Queryable, Debug)]
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
    fn save(self, connection: &mut PgConnection) -> QueryResult<User> {
        diesel::insert_into(users::dsl::users)
            .values(self)
            .get_result(connection)
    }
}

impl User {
    /// Convert to the public version
    pub fn to_public(self) -> UserPublic {
        UserPublic {
            id: self.id,
            username: self.username,
        }
    }

    /// A function that loads a user from the database and checks the password hash
    pub fn fetch_check(
        connection: &mut PgConnection,
        provided_username: String,
        provided_password: String,
    ) -> Option<User> {
        use crate::schema::users::dsl::*;
        let user = users
            .filter(username.eq(provided_username))
            .load::<User>(connection)
            .expect("Error loading users")
            .pop();

        if let Some(user) = user {
            // Hash the password
            let password_verifier = Argon2::default();
            let hash = PasswordHash::new(&user.password_hash).expect("Could not hash the password");

            // Check the password hash
            let is_password_correct = password_verifier
                .verify_password(provided_password.as_bytes(), &hash)
                .is_ok();

            // Only return a user if the password is correct
            if is_password_correct {
                Some(user)
            } else {
                None
            }
        } else {
            None
        }
    }
}
