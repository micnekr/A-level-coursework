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

#[derive(Identifiable, Queryable, Debug, Serialize)]
/// A user struct that represents a user record in a database
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

impl UnsavedUser {
    /// creates a password hash for the user using the salt and the hashing algorithm
    pub fn hash(text: &str) -> password_hash::Result<String> {
        // Generate a unique salt
        let salt = SaltString::generate(&mut OsRng);

        // Create the instance of the hashing algorithm
        let argon2 = Argon2::default();

        // Do the hashing
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
    /// Saves a user to the database and returns an object based on the new user stored in the database
    fn save(self, connection: &mut PgConnection) -> QueryResult<User> {
        diesel::insert_into(users::dsl::users)
            .values(self)
            .get_result(connection)
    }
}

impl User {
    /// A function that loads a user from the database and checks the password hash
    pub fn fetch_check(
        connection: &mut PgConnection,
        provided_username: String,
        provided_password: String,
    ) -> Option<User> {
        use crate::schema::users::dsl::*;
        // Find all the users by the username
        // Note that we only expect one, so we pop once
        let user = users
            .filter(username.eq(provided_username))
            .load::<User>(connection)
            .expect("Error loading users")
            .pop();

        // If the user was found
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
            // If no user was found in the database, fail
        } else {
            None
        }
    }
}
