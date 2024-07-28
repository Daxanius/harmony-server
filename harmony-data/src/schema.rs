// @generated automatically by Diesel CLI.

diesel::table! {
    playlists (id) {
        id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        owner -> Int4,
        public -> Bool,
        created -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    playlists_songs (id) {
        id -> Int4,
        playlist -> Int4,
        song -> Int4,
        created -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    songs (id) {
        id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        #[max_length = 255]
        author -> Nullable<Varchar>,
        added_by -> Int4,
        #[max_length = 255]
        youtube_url -> Varchar,
        #[max_length = 60]
        file_id -> Varchar,
        created -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 16]
        name -> Varchar,
        #[max_length = 255]
        hash -> Varchar,
        admin -> Bool,
        created -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(playlists -> users (owner));
diesel::joinable!(playlists_songs -> playlists (playlist));
diesel::joinable!(playlists_songs -> songs (song));
diesel::joinable!(songs -> users (added_by));

diesel::allow_tables_to_appear_in_same_query!(playlists, playlists_songs, songs, users,);
