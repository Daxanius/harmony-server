use crate::model::playlist_song::PlaylistSongModel;
#[allow(clippy::module_name_repetitions)]
use crate::schema::playlists;
use crate::schema::playlists::dsl::*;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(Queryable, Selectable, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = crate::schema::playlists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PlaylistModel {
    pub id: i32,
    pub name: String,
    pub owner: i32,
    pub public: bool,
    pub created: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = playlists)]
pub struct NewPlaylistModel {
    pub name: String,
    pub owner: i32,
}

impl PlaylistModel {
    /// Attempts to create a playlist
    /// # Errors
    /// When the database operation fails
    pub fn create(playlist: &NewPlaylistModel) -> Result<PlaylistModel, crate::result::Error> {
        let connection: &mut PgConnection = &mut crate::establish_connection();

        let result = diesel::insert_into(playlists::table)
            .values(playlist)
            .get_result::<PlaylistModel>(connection);

        match result {
            Ok(playlist) => Ok(playlist),
            Err(err) => Err(crate::result::Error::Database(err.to_string())),
        }
    }

    /// Removes a playlist
    /// # Errors
    /// When the database operation fails
    pub fn remove(playlist_id: i32) -> Result<(), crate::result::Error> {
        let connection = &mut crate::establish_connection();

        PlaylistSongModel::remove_all_songs_from_playlist(playlist_id)?;
        let result = diesel::delete(playlists.filter(id.eq(playlist_id))).execute(connection);

        match result {
            Ok(_) => Ok(()),
            Err(err) => match err {
                diesel::result::Error::NotFound => Err(crate::result::Error::NotFound),
                _ => Err(crate::result::Error::Database(err.to_string())),
            },
        }
    }

    /// Attempts to get a playlist by id
    /// # Errors
    /// When the playlist does not exist, or the database operation fails
    pub fn get_by_id(playlist_d: i32) -> Result<PlaylistModel, crate::result::Error> {
        let connection: &mut PgConnection = &mut crate::establish_connection();

        let result = playlists
            .find(playlist_d)
            .first::<PlaylistModel>(connection);

        match result {
            Ok(playlist) => Ok(playlist),
            Err(err) => match err {
                diesel::result::Error::NotFound => Err(crate::result::Error::NotFound),
                _ => Err(crate::result::Error::Database(err.to_string())),
            },
        }
    }

    /// Find a playlist by name
    /// # Errors
    /// When the database operation fails
    pub fn find_by_name(
        playlist_name: &str,
        calling_user: i32,
    ) -> Result<Vec<PlaylistModel>, crate::result::Error> {
        let connection = &mut crate::establish_connection();

        // Perform case-insensitive search
        let result = playlists
            .order(name.desc())
            .filter(
                name.ilike(format!("%{playlist_name}%"))
                    .and(public.eq(true).or(owner.eq(calling_user))),
            )
            .load::<PlaylistModel>(connection);

        match result {
            Ok(playlist_list) => Ok(playlist_list),
            Err(err) => Err(crate::result::Error::Database(err.to_string())),
        }
    }

    /// Find a playlist by user
    /// # Errors
    /// When the database operation fails
    pub fn find_by_user(
        user_id: i32,
        calling_user: i32,
    ) -> Result<Vec<PlaylistModel>, crate::result::Error> {
        let connection = &mut crate::establish_connection();

        // Perform case-insensitive search
        let result = playlists
            .order(name.desc())
            .filter(
                owner
                    .eq(user_id)
                    .and(public.eq(true).or(owner.eq(calling_user))),
            )
            .load::<PlaylistModel>(connection);

        match result {
            Ok(playlist_list) => Ok(playlist_list),
            Err(err) => Err(crate::result::Error::Database(err.to_string())),
        }
    }

    /// Gets a list of songs
    /// # Errors
    /// When the database operation fails
    pub fn get_list(calling_user: i32) -> Result<Vec<PlaylistModel>, crate::result::Error> {
        let connection = &mut crate::establish_connection();

        // Perform case-insensitive search
        let result = playlists
            .order(name.desc())
            .filter(public.eq(true).or(owner.eq(calling_user)))
            .load::<PlaylistModel>(connection);

        match result {
            Ok(playlist_list) => Ok(playlist_list),
            Err(err) => Err(crate::result::Error::Database(err.to_string())),
        }
    }
}
