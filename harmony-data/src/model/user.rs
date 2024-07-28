#[allow(clippy::module_name_repetitions)]
use crate::schema::users;
use crate::schema::users::dsl::*;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Queryable, Selectable, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserModel {
    pub id: i32,
    pub name: String,
    pub hash: String,
    pub admin: bool,
    pub created: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUserModel {
    pub name: String,
    pub hash: String,
}

impl UserModel {
    /// Attempts to create a user
    /// # Errors
    /// When the database operation fails
    pub fn create(user: NewUserModel) -> Result<UserModel, crate::result::Error> {
        let connection: &mut PgConnection = &mut crate::establish_connection();

        let result = diesel::insert_into(users::table)
            .values(&user)
            .get_result::<UserModel>(connection);

        match result {
            Ok(user) => Ok(user),
            Err(err) => Err(crate::result::Error::Database(err.to_string())),
        }
    }

    /// Returns no error if the user exists
    /// # Errors
    /// When the user does not exist or the database operation fails
    pub fn exists(username: &String) -> Result<(), crate::result::Error> {
        let connection: &mut PgConnection = &mut crate::establish_connection();

        let result = users
            .filter(name.ilike(format!("%{username}%")))
            .first::<UserModel>(connection);

        match result {
            Ok(_) => Ok(()),
            Err(err) => match err {
                diesel::result::Error::NotFound => Err(crate::result::Error::NotFound),
                _ => Err(crate::result::Error::Database(err.to_string())),
            },
        }
    }

    /// Find a user by id
    /// # Errors
    /// When the user does not exist or the database operation fails
    pub fn get_by_id(user_id: i32) -> Result<UserModel, crate::result::Error> {
        let connection: &mut PgConnection = &mut crate::establish_connection();

        let result = users.find(user_id).first::<UserModel>(connection);

        match result {
            Ok(user) => Ok(user),
            Err(err) => match err {
                diesel::result::Error::NotFound => Err(crate::result::Error::NotFound),
                _ => Err(crate::result::Error::Database(err.to_string())),
            },
        }
    }

    /// Find a user by name
    /// # Errors
    /// When the user does not exist or the database operation fails
    pub fn get_by_name(username: &String) -> Result<UserModel, crate::result::Error> {
        let connection = &mut crate::establish_connection();

        // Perform case-insensitive search
        let result = users
            .order(name.desc())
            .filter(name.eq(username))
            .first::<UserModel>(connection);

        match result {
            Ok(user) => Ok(user),
            Err(err) => Err(crate::result::Error::Database(err.to_string())),
        }
    }

    /// Find a user by name
    /// # Errors
    /// When the database operation fails
    pub fn find(username: &str) -> Result<Vec<UserModel>, crate::result::Error> {
        let connection = &mut crate::establish_connection();

        // Perform case-insensitive search
        let result = users
            .order(name.desc())
            .filter(name.ilike(format!("%{username}%")))
            .load::<UserModel>(connection);

        match result {
            Ok(user_list) => Ok(user_list),
            Err(err) => Err(crate::result::Error::Database(err.to_string())),
        }
    }

    /// Gets a list of users
    /// # Errors
    /// When the database operation fails
    pub fn get_list() -> Result<Vec<UserModel>, crate::result::Error> {
        let connection = &mut crate::establish_connection();

        // Perform case-insensitive search
        let result = users
            .order(name.desc())
            .select(users::all_columns)
            .load::<UserModel>(connection);

        match result {
            Ok(user_list) => Ok(user_list),
            Err(err) => Err(crate::result::Error::Database(err.to_string())),
        }
    }
}
