#[allow(clippy::module_name_repetitions)]
use crate::model::song::SongModel;
use crate::schema::playlists_songs::dsl::*;
use crate::schema::{playlists_songs, songs};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(Queryable, Selectable, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = crate::schema::playlists_songs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PlaylistSongModel {
    pub id: i32,
    pub playlist: i32,
    pub song: i32,
    pub created: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = playlists_songs)]
pub struct NewPlaylistSongModel {
    pub playlist: i32,
    pub song: i32,
}

impl PlaylistSongModel {
    /// Adds a song to a playlist
    /// # Errors
    /// When the database operation fails
    pub fn add_song_to_playlist(
        playlist_song: NewPlaylistSongModel,
    ) -> Result<PlaylistSongModel, crate::result::Error> {
        let connection = &mut crate::establish_connection();

        // Perform case-insensitive search
        let result = diesel::insert_into(playlists_songs::table)
            .values(playlist_song)
            .get_result::<PlaylistSongModel>(connection);
        match result {
            Ok(playlist_song) => Ok(playlist_song),
            Err(err) => match err {
                diesel::result::Error::NotFound => Err(crate::result::Error::NotFound),
                _ => Err(crate::result::Error::Database(err.to_string())),
            },
        }
    }

    /// Adds a song to a playlist
    /// # Errors
    /// When the database operation fails
    pub fn remove_song_from_playlist(
        playlist_id: i32,
        song_id: i32,
    ) -> Result<(), crate::result::Error> {
        let connection = &mut crate::establish_connection();

        // Perform case-insensitive search
        let result =
            diesel::delete(playlists_songs.filter(playlist.eq(playlist_id).and(song.eq(song_id))))
                .execute(connection);

        match result {
            Ok(_) => Ok(()),
            Err(err) => match err {
                diesel::result::Error::NotFound => Err(crate::result::Error::NotFound),
                _ => Err(crate::result::Error::Database(err.to_string())),
            },
        }
    }

    /// Removes all songs from a playlist
    /// # Errors
    /// When the database operation fails
    pub fn remove_all_songs_from_playlist(playlist_id: i32) -> Result<(), crate::result::Error> {
        let connection = &mut crate::establish_connection();

        // Perform case-insensitive search
        let result =
            diesel::delete(playlists_songs.filter(playlist.eq(playlist_id))).execute(connection);

        match result {
            Ok(_) => Ok(()),
            Err(err) => match err {
                diesel::result::Error::NotFound => Err(crate::result::Error::NotFound),
                _ => Err(crate::result::Error::Database(err.to_string())),
            },
        }
    }

    pub fn song_in_playlist(playlist_id: i32, song_id: i32) -> Result<(), crate::result::Error> {
        let connection = &mut crate::establish_connection();

        // Perform case-insensitive search
        let result = playlists_songs
            .filter(playlist.eq(playlist_id).and(song.eq(song_id)))
            .first::<PlaylistSongModel>(connection);

        match result {
            Ok(_) => Ok(()),
            Err(err) => match err {
                diesel::result::Error::NotFound => Err(crate::result::Error::NotFound),
                _ => Err(crate::result::Error::Database(err.to_string())),
            },
        }
    }

    /// Gets all songs from a playlist
    /// # Errors
    /// When the database operation fails
    pub fn get_songs_from_playlist(
        playlist_id: i32,
    ) -> Result<Vec<SongModel>, crate::result::Error> {
        let connection = &mut crate::establish_connection();

        // Join playlists_songs with songs to get the song details
        let query = playlists_songs::table
            .inner_join(songs::table)
            .filter(playlist.eq(playlist_id))
            .select(songs::all_columns) // Adjust this to select specific columns if needed
            .load::<SongModel>(connection);

        match query {
            Ok(songs) => Ok(songs),
            Err(err) => match err {
                diesel::result::Error::NotFound => Err(crate::result::Error::NotFound),
                _ => Err(crate::result::Error::Database(err.to_string())),
            },
        }
    }
}
