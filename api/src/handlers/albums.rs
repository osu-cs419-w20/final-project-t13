use std::collections::BTreeSet;

use crate::Error;
use crate::db::DB;
use crate::filters::{RelationsOption, PaginationOptions};

#[derive(Serialize)]
pub struct Album {
    pub id: i32,
    pub mbid: String,
    pub title: String,
    pub artist_id: i32,
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<super::artists::Artist>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracks: Option<Vec<super::tracks::Track>>,
}

// GET /albums(?page=X&limit=Y)
pub async fn get_albums(opts: PaginationOptions, db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    let client = db.get().await?;
    let count = db.count_rows("album", &client).await.map_err(Error::from)?.unwrap_or(0);
    let (limit, page) = match (opts.limit.or(Some(15)), opts.page) {
        (Some(l), Some(p)) => {
            if count < p * l {
                (l, (count as f64 / l as f64).ceil() as i64)
            } else {
                (l, p)
            }
        }
        (Some(l), None) => (l, 1),
        _ => (15, 1),
    };
    let offset = (page - 1) * limit as i64;
    let total_pages = (count as f64 / limit as f64).ceil() as i64;
    let stmt = client.prepare("
        SELECT A.id, A.mbid, A.title, A.artist_id, I.url as image_url
        FROM album A
        INNER JOIN album_image I
            ON I.album_id = A.id
        LIMIT $1 OFFSET $2
    ").await.map_err(Error::from)?;
    let rows = client.query(&stmt, &[&limit, &offset]).await.map_err(Error::from)?;
    let mut albums = Vec::new();

    for row in rows {
        let album = Album {
            id: row.get(0),
            mbid: row.get(1),
            title: row.get(2),
            artist_id: row.get(3),
            image_url: row.get(4),
            artist: None,
            tracks: None,
        };
        albums.push(album);
    }

    let res = super::PaginatedResponse {
        page,
        count,
        total_pages,
        data: albums,
    };

    Ok(warp::reply::json(&res))
}

// GET /albums/:id
pub async fn get_album_with_id(id: i32, rels: RelationsOption, db: DB) -> Result<impl warp::Reply, warp::Rejection> {
    let client = db.get().await?;
    let rels = rels.relations.unwrap_or(BTreeSet::new());
    let mut select_fields = vec!["A.id", "A.mbid", "A.title", "A.artist_id", "I.url as image_url"];
    let mut joins = vec!["INNER JOIN album_image I ON I.album_id = A.id"];
    let mut loading_artist = false;
    let mut loading_tracks = false;

    for rel in rels {
        use crate::filters::Relation;
        match rel {
            Relation::Artist => {
                select_fields.extend_from_slice(&[
                    "C.id",
                    "C.mbid",
                    "C.name",
                    "CI.url as artist_image_url",
                ]);
                joins.extend_from_slice(&[
                    "INNER JOIN artist C ON C.id = A.artist_id",
                    "LEFT OUTER JOIN artist_image CI ON CI.artist_id = C.id",
                ]);
                loading_artist = true;
            }
            Relation::Tracks => {
                select_fields.extend_from_slice(&[
                    "T.id",
                    "T.mbid",
                    "T.title",
                    "T.position",
                    "T.bit_rate",
                    "T.duration",
                    "T.file_location",
                    "T.album_id",
                ]);
                joins.extend_from_slice(&[
                    "LEFT OUTER JOIN track T ON T.album_id = A.id",
                ]);
                loading_tracks = true;
            }
            _ => {}
        }
    }

    let select_fields = select_fields.as_slice().join(", ");
    let joins = joins.as_slice().join("\n");
    let q = format!("SELECT {} FROM album A {} WHERE A.id = $1", select_fields, joins);
    let stmt = client.prepare(&q).await.map_err(Error::from)?;
    let rows = client.query(&stmt, &[&id]).await.map_err(Error::from)?;

    if rows.len() < 1 {
        return Err(warp::reject());
    }

    let mut tracks = None;

    if loading_tracks {
        let mut tracks_vec = Vec::new();
        for row in &rows {
            // If there's no ID, no track record exists on this row, so
            // skip it.
            if let Err(_) = row.try_get::<'_, _, i32>(9) {
                continue;
            }
            // If artist is being loaded, it will always be loaded before tracks
            let offset = if loading_artist { 4 } else { 0 };
            let track = crate::handlers::tracks::Track {
                id: row.get(5 + offset),
                mbid: row.get(6 + offset),
                title: row.get(7 + offset),
                position: row.get(8 + offset),
                bit_rate: row.get(9 + offset),
                duration: row.get(10 + offset),
                file_location: row.get(11 + offset),
                album_id: row.get(12 + offset),
            };
            tracks_vec.push(track);
        }

        if !tracks_vec.is_empty() {
            tracks = Some(tracks_vec);
        }
    }

    let row = &rows[0];
    let artist = if loading_artist {
        Some(crate::handlers::artists::Artist {
            id: row.get(5),
            mbid: row.get(6),
            name: row.get(7),
            image_url: row.get(8),
            albums: None,
        })
    } else { None };
    let album = Album {
        id: row.get(0),
        mbid: row.get(1),
        title: row.get(2),
        artist_id: row.get(3),
        image_url: row.get(4),
        artist,
        tracks,
    };

    Ok(warp::reply::json(&album))
}
