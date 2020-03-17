use std::collections::BTreeSet;

use warp::http::StatusCode;

use crate::Error;
use crate::db::DB;
use crate::filters::RelationsOption;

#[derive(Serialize)]
pub struct Playlist {
    id: i32,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracks: Option<Vec<PlaylistTrack>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistTrack {
    id: i32,
    title: String,
    duration: i32,
    position: i32,
    album_id: i32,
    album_title: String,
    album_image: String,
    artist_id: i32,
    artist_name: String,
}

#[derive(Deserialize)]
pub struct NewPlaylist {
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToPlaylist {
    track_id: i32,
}

pub async fn get_playlists(db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    let client = db.get().await?;
    let stmt = client.prepare("
        SELECT id, name
        FROM playlist
        ORDER BY name ASC
    ").await.map_err(Error::from)?;
    let rows = client.query(&stmt, &[]).await.map_err(Error::from)?;
    let mut playlists = Vec::new();

    for row in rows {
        let playlist = Playlist {
            id: row.get(0),
            name: row.get(1),
            tracks: None,
        };
        playlists.push(playlist);
    }

    Ok(warp::reply::json(&playlists))
}

pub async fn create_playlist(p: NewPlaylist, db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    let client = db.get().await?;
    let stmt = client.prepare("
        INSERT INTO playlist (name)
        VALUES ($1)
        RETURNING id, name
    ").await.map_err(Error::from)?;
    let rows = client.query(&stmt, &[&p.name]).await.map_err(Error::from)?;

    if rows.is_empty() {
        return Err(warp::reject());
    }

    let row = &rows[0];
    let playlist = Playlist {
        id: row.get(0),
        name: row.get(1),
        tracks: None,
    };

    Ok(warp::reply::json(&playlist))
}

pub async fn get_playlist_with_id(id: i32, rels: RelationsOption, db: DB) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let rels = rels.relations.unwrap_or(BTreeSet::new());
    let mut select_fields = vec!["P.id", "P.name"];
    let mut joins = Vec::new();

    let mut loading_tracks = false;
    for rel in rels {
        use crate::filters::Relation;
        match rel {
            Relation::Tracks => {
                select_fields.extend_from_slice(&[
                    "T.id",
                    "T.title",
                    "T.duration",
                    "PT.position",
                    "R.id",
                    "R.title",
                    "R.image_url",
                    "A.id",
                    "A.name",
                ]);
                joins.extend_from_slice(&[
                    "LEFT OUTER JOIN playlist_track PT ON PT.playlist_id = P.id",
                    "LEFT OUTER JOIN track T ON T.id = PT.track_id",
                    "LEFT OUTER JOIN album R ON R.id = T.album_id",
                    "LEFT OUTER JOIN artist A ON A.id = R.artist_id",
                ]);
                loading_tracks = true;
            }
            _ => {}
        }
    }

    let select_fields = select_fields.as_slice().join(", ");
    let joins = joins.as_slice().join("\n");
    let q = format!("SELECT {} FROM playlist P {} WHERE P.id = $1", select_fields, joins);
    let client = db.get().await?;
    let stmt = client.prepare(&q).await.map_err(Error::from)?;
    let rows = client.query(&stmt, &[&id]).await.map_err(Error::from)?;

    if rows.is_empty() {
        return Ok(Box::new(warp::reply::with_status(warp::reply(), StatusCode::NOT_FOUND)));
    }

    let mut tracks = None;

    if loading_tracks {
        let mut playlist_tracks = Vec::new();
        for row in &rows {
            let track = PlaylistTrack {
                id: match row.try_get(2) {
                    Ok(id) => id,
                    Err(_) => break,
                },
                title: row.get(3),
                duration: row.get(4),
                position: row.get(5),
                album_id: row.get(6),
                album_title: row.get(7),
                album_image: row.get(8),
                artist_id: row.get(9),
                artist_name: row.get(10),
            };
            playlist_tracks.push(track);
        }

        tracks = Some(playlist_tracks);
    }

    let row = &rows[0];
    let playlist = Playlist {
        id: row.get(0),
        name: row.get(1),
        tracks,
    };

    Ok(Box::new(warp::reply::json(&playlist)))
}

pub async fn add_to_playlist(id: i32, t: AddToPlaylist, db: DB) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let client = db.get().await?;
    let stmt = client.prepare("
        SELECT MAX(position) FROM playlist_track WHERE playlist_id = $1
    ").await.map_err(Error::from)?;
    let rows = client.query(&stmt, &[&id]).await.map_err(Error::from)?;

    let next_pos = if rows.is_empty() { 1 } else { rows[0].try_get::<'_, _, i32>(0).unwrap_or(0) + 1 };
    let stmt = client.prepare("
        INSERT INTO playlist_track (playlist_id, track_id, position)
        VALUES ($1, $2, $3)
    ").await.map_err(Error::from)?;
    client.query(&stmt, &[&id, &t.track_id, &next_pos]).await.map_err(Error::from)?;

    let stmt = client.prepare("
        SELECT T.id, T.title, T.duration, PT.position, R.id, R.title, R.image_url, A.id, A.name
        FROM track T
        INNER JOIN playlist_track PT ON PT.playlist_id = $1 AND PT.track_id = T.id
        INNER JOIN album R ON R.id = T.album_id
        INNER JOIN artist A ON A.id = R.artist_id
    ").await.map_err(Error::from)?;
    let rows = client.query(&stmt, &[&id]).await.map_err(Error::from)?;

    if rows.is_empty() {
        return Ok(Box::new(warp::reply::with_status(warp::reply(), StatusCode::INTERNAL_SERVER_ERROR)));
    }

    let row = &rows[0];
    let track = PlaylistTrack {
        id: row.get(0),
        title: row.get(1),
        duration: row.get(2),
        position: row.get(3),
        album_id: row.get(4),
        album_title: row.get(5),
        album_image: row.get(6),
        artist_id: row.get(7),
        artist_name: row.get(8),
    };

    Ok(Box::new(warp::reply::with_status(warp::reply::json(&track), StatusCode::CREATED)))
}
