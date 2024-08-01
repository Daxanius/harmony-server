#[allow(clippy::module_name_repetitions)]
use crate::schema::songs;
use crate::schema::songs::dsl::*;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(Queryable, Selectable, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = crate::schema::songs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SongModel {
    pub id: i32,
    pub name: String,
    pub author: Option<String>,
    pub added_by: i32,
    pub youtube_url: String,
    pub file_id: String,
    pub created: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = songs)]
pub struct NewSongModel {
    pub name: String,
    pub author: Option<String>,
    pub added_by: i32,
    pub youtube_url: String,
    pub file_id: String,
}

impl SongModel {
    /// Attempts to create a song
    /// # Errors
    /// When the database operation fails
    pub fn create(song: &NewSongModel) -> Result<SongModel, crate::result::Error> {
        let connection: &mut PgConnection = &mut crate::establish_connection();

        let result = diesel::insert_into(songs::table)
            .values(song)
            .get_result::<SongModel>(connection);

        match result {
            Ok(song) => Ok(song),
            Err(err) => Err(crate::result::Error::Database(err.to_string())),
        }
    }

    /// Returns no error if the song does not exist
    /// # Errors
    /// When the song does exist or the database operation fails
    pub fn exists(url: &str) -> Result<(), crate::result::Error> {
        let connection: &mut PgConnection = &mut crate::establish_connection();

        let result = songs
            .filter(youtube_url.like(format!("%{url}%")))
            .first::<SongModel>(connection);

        match result {
            Ok(_) => Ok(()),
            Err(err) => match err {
                diesel::result::Error::NotFound => Err(crate::result::Error::NotFound),
                _ => Err(crate::result::Error::Database(err.to_string())),
            },
        }
    }

    /// Attempts to get a song by id
    /// # Errors
    /// When the user does not exist, or the database operation fails
    pub fn get_by_id(song_id: i32) -> Result<SongModel, crate::result::Error> {
        let connection: &mut PgConnection = &mut crate::establish_connection();

        let result = songs.find(song_id).first::<SongModel>(connection);

        match result {
            Ok(song) => Ok(song),
            Err(err) => match err {
                diesel::result::Error::NotFound => Err(crate::result::Error::NotFound),
                _ => Err(crate::result::Error::Database(err.to_string())),
            },
        }
    }

    /// Find a song by user or author
    /// # Errors
    /// When the database operation fails
    pub fn find(query: &str) -> Result<Vec<SongModel>, crate::result::Error> {
        let connection: &mut PgConnection = &mut crate::establish_connection();

        // Perform case-insensitive search
        let result = songs
            .order(name.desc())
            .filter(
                name.ilike(format!("%{query}%"))
                    .or(author.ilike(format!("%{query}%"))),
            )
            .load::<SongModel>(connection);

        match result {
            Ok(song_list) => Ok(song_list),
            Err(err) => Err(crate::result::Error::Database(err.to_string())),
        }
    }

    /// List all songs
    /// # Errors
    /// When the database operation fails
    pub fn get_list() -> Result<Vec<SongModel>, crate::result::Error> {
        let connection: &mut PgConnection = &mut crate::establish_connection();

        // Perform case-insensitive search
        let result = songs
            .order(name.desc())
            .select(songs::all_columns)
            .load::<SongModel>(connection);

        match result {
            Ok(song_list) => Ok(song_list),
            Err(err) => Err(crate::result::Error::Database(err.to_string())),
        }
    }
}
