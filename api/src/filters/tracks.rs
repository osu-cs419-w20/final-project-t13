use warp::Filter;

use crate::db::DB;
use crate::handlers::tracks::{
    get_track_with_id,
    play_track,
};

pub(super) fn tracks_filters(db: DB)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    get_track_with_id_filter(db.clone())
        .or(play_track_filter(db))
}

fn get_track_with_id_filter(db: DB)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("track" / i32)
        .and(warp::get())
        .and(super::db_filter(db))
        .and_then(get_track_with_id)
}

fn play_track_filter(db: DB)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("play" / i32)
        .and(warp::get())
        .and(super::db_filter(db))
        .and_then(play_track)
}
