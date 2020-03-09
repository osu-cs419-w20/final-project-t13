use crate::Error;
use crate::db::DB;

#[derive(Serialize)]
pub struct Track {
    pub id: i32,
    pub mbid: String,
    pub title: String,
    pub position: i32,
    pub bit_rate: i32,
    pub duration: i32,
    pub file_location: String,
    pub album_id: i32,
}

pub async fn get_track_with_id(id: i32, db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    let client = db.get().await?;
    let stmt = client.prepare("
        SELECT 
    ").await.map_err(Error::from)?;
    let rows = client.query(&stmt, &[&id]).await.map_err(Error::from)?;

    if rows.len() < 1 {
        return Err(warp::reject());
    }

    let row = &rows[0];
    let track = Track {
        id: row.get(0),
        mbid: row.get(1),
        title: row.get(2),
        position: row.get(3),
        bit_rate: row.get(4),
        duration: row.get(5),
        file_location: row.get(6),
        album_id: row.get(7),
    };

    Ok(warp::reply::json(&track))
}

pub async fn play_track(id: i32, db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    let client = db.get().await?;
    let stmt = client.prepare("
        SELECT file_location
        FROM track
        WHERE id = $1
    ").await.map_err(Error::from)?;
    let rows = client.query(&stmt, &[&id]).await.map_err(Error::from)?;

    if rows.len() < 1 {
        return Err(warp::reject());
    }

    let file_location: String = rows[0].get(0);
    Ok(warp::redirect::temporary(file_location.parse::<warp::http::Uri>().unwrap()))
}
