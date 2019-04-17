//! The `user` module implements different representations of the user model.
//! Different representations are required for different workflows, e.g. to save
//! users to the database, or to expose them in the public API.

use crate::schema::users;
use chrono::NaiveDateTime;
use uuid::Uuid;

/// The `password` module implements the methods to hash and verify a user's
/// password. It is a thin wrapper around the underlying cryptographic library.
pub mod password {
    /// Hash a password.
    ///
    /// This method takes a password, and returns its cryptographic hash.
    pub fn hash(password: &str) -> String {
        bcrypt::hash(password, bcrypt::DEFAULT_COST).expect("Failed to hash password")
    }

    /// Verify a password.
    ///
    /// This method compares a password and a hash, and returns true if the
    /// password matches the hash.
    pub fn verify(password: &str, hash: &str) -> bool {
        bcrypt::verify(password, hash).expect("Failed to verify password hash")
    }
}

/// The `User` struct represents the user table in the database.
#[derive(Queryable)]
pub struct User {
    /// The `id` is used internally to identify users, and to associate them
    /// with other resources.
    pub id: i32,

    /// The `uuid` is used in public views, e.g. in the API.
    pub uuid: Uuid,

    /// The user is uniquely identified by their `email` address.
    pub email: String,

    /// The user's password is hashed using bcrypt, and the hash is stored in
    /// the database.
    pub encrypted_password: String,

    /// The date and time the user record was created.
    pub created_at: NaiveDateTime,

    /// The date and time the user record was last updated.
    pub updated_at: NaiveDateTime,
}

/// The `NewUser` struct is used to create a new user in the database. Create a
/// new user with the user's email address, and a hash of their password.
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    email: &'a String,
    encrypted_password: &'a String,
}

#[cfg(test)]
mod tests {
    use crate::config::{Config, Environment};
    use crate::models::user::password::hash;
    use crate::models::user::User;
    use crate::schema::users::dsl::*;
    use diesel::insert_into;
    use diesel::prelude::*;

    fn establish_connection() -> PgConnection {
        let config = Config {
            env: Environment::Test,
            ..Default::default()
        };

        let connection =
            PgConnection::establish(&config.database_url()).expect("Error connecting to database");

        connection
            .begin_test_transaction()
            .expect("Error starting test transaction");

        connection
    }

    #[test]
    fn read_user() {
        let connection = establish_connection();

        let email_value = String::from("test@example.com");
        let password = String::from("password");
        let encrypted_password_value = hash(&password);

        insert_into(users)
            .values((
                email.eq(&email_value),
                encrypted_password.eq(&encrypted_password_value),
            ))
            .execute(&connection)
            .expect("Failed to set up test");

        let mut items = users
            .filter(email.eq(&email_value))
            .limit(1)
            .load::<User>(&connection)
            .expect("Failed to query user");

        let user = items.pop().expect("Failed to load user");

        assert_eq!(user.encrypted_password, encrypted_password_value);
    }
}
