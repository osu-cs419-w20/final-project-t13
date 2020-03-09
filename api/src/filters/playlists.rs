use warp::Filter;

use crate::db::DB;
use crate::handlers::playlists::{
    get_playlists,
    create_playlist,
    get_playlist_with_id,
    add_to_playlist,
};

pub(super) fn playlists_filters(db: DB)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    get_playlists_filter(db.clone())
        .or(create_playlist_filter(db.clone()))
        .or(get_playlist_with_id_filter(db.clone()))
        .or(add_to_playlist_filter(db))
}

fn get_playlists_filter(db: DB)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("playlists")
        .and(warp::get())
        .and(super::db_filter(db))
        .and_then(get_playlists)
}

fn create_playlist_filter(db: DB)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("playlists")
        .and(warp::post())
        .and(warp::body::json())
        .and(super::db_filter(db))
        .and_then(create_playlist)
}

fn get_playlist_with_id_filter(db: DB)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("playlists" / i32)
        .and(warp::get())
        .and(warp::query::<super::RelationsOption>())
        .and(super::db_filter(db))
        .and_then(get_playlist_with_id)
}

fn add_to_playlist_filter(db: DB)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("playlists" / i32)
        .and(warp::post())
        .and(warp::body::json())
        .and(super::db_filter(db))
        .and_then(add_to_playlist)
}
