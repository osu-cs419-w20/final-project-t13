use warp::Filter;

use crate::db::DB;
use crate::handlers::albums::{
    get_album_with_id,
    get_albums,
};

pub(super) fn albums_filters(db: DB)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    get_albums_filter(db.clone())
        .or(get_album_with_id_filter(db))
}

fn get_albums_filter(db: DB)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("albums")
        .and(warp::get())
        .and(warp::query::<super::RelationsOption>())
        .and(warp::query::<super::PaginationOptions>())
        .and(super::db_filter(db))
        .and_then(get_albums)
}

fn get_album_with_id_filter(db: DB)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("albums" / i32)
        .and(warp::get())
        .and(warp::query::<super::RelationsOption>())
        .and(super::db_filter(db))
        .and_then(get_album_with_id)
}
