use warp::Filter;

use crate::db::DB;
use crate::handlers::artists::{
    get_artist_with_id,
    get_artists,
};

pub(super) fn artists_filters(db: DB)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    get_artists_filter(db.clone())
        .or(get_artist_with_id_filter(db))
}

fn get_artists_filter(db: DB)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("artists")
        .and(warp::get())
        .and(warp::query::<super::PaginationOptions>())
        .and(super::db_filter(db))
        .and_then(get_artists)
}

fn get_artist_with_id_filter(db: DB)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("artists" / i32)
        .and(warp::get())
        .and(warp::query::<super::RelationsOption>())
        .and(super::db_filter(db))
        .and_then(get_artist_with_id)
}
