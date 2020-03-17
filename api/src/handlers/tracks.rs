use warp::http::StatusCode;

use crate::Error;
use crate::db::DB;
use crate::handlers::artists::Artist;
use crate::handlers::albums::Album;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<Artist>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<Album>,
}

pub async fn get_track_with_id(id: i32, db: DB) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let select_fields =  &[
        "T.id",
        "T.mbid",
        "T.title",
        "T.position",
        "T.bit_rate",
        "T.duration",
        "T.file_location",
        "T.album_id",
        "R.id",
        "R.mbid",
        "R.title",
        "R.artist_id",
        "R.image_url",
        "A.id",
        "A.mbid",
        "A.name",
        "A.image_url",
    ].join(", ");
    let client = db.get().await?;
    let q = format!("
        SELECT {}
        FROM track T
        INNER JOIN album R ON R.id = T.album_id
        INNER JOIN artist A ON A.id = R.artist_id
        WHERE T.id = $1
    ", select_fields);
    let stmt = client.prepare(&q).await.map_err(Error::from)?;
    let rows = client.query(&stmt, &[&id]).await.map_err(Error::from)?;

    if rows.len() < 1 {
        return Ok(Box::new(warp::reply::with_status(warp::reply(), StatusCode::NOT_FOUND)));
    }

    let row = &rows[0];
    let album = Album {
        id: row.get(8),
        mbid: row.get(9),
        title: row.get(10),
        artist_id: row.get(11),
        image_url: row.get(12),
        artist: None,
        tracks: None,
    };
    let artist = Artist {
        id: row.get(13),
        mbid: row.get(14),
        name: row.get(15),
        image_url: row.get(16),
        albums: None,
    };
    let track = Track {
        id: row.get(0),
        mbid: row.get(1),
        title: row.get(2),
        position: row.get(3),
        bit_rate: row.get(4),
        duration: row.get(5),
        file_location: row.get(6),
        album_id: row.get(7),
        album: Some(album),
        artist: Some(artist),
    };

    Ok(Box::new(warp::reply::json(&track)))
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

    let file_location = format!("/api/static{}", rows[0].get::<_, String>(0));
    let redir_location = crate::encoding::encode_uri(&file_location);
    Ok(warp::redirect::temporary(redir_location.parse::<warp::http::Uri>().unwrap()))
}
