use std::env;

use tokio::time::{delay_for, Duration};
use deadpool_postgres::{Config, Manager, Pool};
use tokio_postgres::{NoTls};
use walkdir::WalkDir;

mod av;
mod import;
mod metadata;
mod models;
mod utils;

use metadata::providers::{MBClient, SpotifyClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let music_dir = env::var("MUSIC_DIR_ROOT").expect("MUSIC_DIR_ROOT environment variable not set");

    let pool = create_pool()?;

    let mb_client = MBClient::new()?;
    let spotify_client = SpotifyClient::new();

    let walker = WalkDir::new(music_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .filter(|e| match e.path().extension() {
            Some(ext) => {
                match ext.to_str() {
                    Some("flac") | Some("mp3") => true,
                    _ => false,
                }
            }
            None => false,
        });
    for entry in walker {
        let c = entry.path().to_str().unwrap();
        let track: av::metadata::Track<'_> = av::metadata::Track::new(c)?;
        let imp = import::TrackImporter::new(mb_client.clone(), spotify_client.clone(), track);
        let (ac, rel, rec) = imp.find_match().await;

        println!("{} {} {}", ac.artist.id, rel.id, rec.id.as_ref().unwrap());

        let (mut existing_artist, mut existing_album) =
            existing_artist_album(&rel.id, &ac.artist.id, pool.clone()).await?;

        let mut client = pool.get().await?;
        let tx = client.transaction().await?;

        if existing_artist.is_none() {
            let artist = imp.build_artist(&ac).await;
            let insert_artist_stmt = tx.prepare("INSERT INTO artist (mbid, name, image_url) VALUES ($1, $2, $3) RETURNING id").await?;
            let row = tx.query(&insert_artist_stmt, &[&artist.mbid, &artist.name, &artist.image_url]).await?;
            let artist_id: i32 = row[0].get(0);
            existing_artist = Some(artist_id);
        }

        if existing_album.is_none() {
            let album = imp.build_album(&rel, &ac).await;
            let insert_album_stmt = tx.prepare("INSERT INTO album (mbid, title, image_url, artist_id) VALUES ($1, $2, $3, $4) RETURNING id").await?;
            let row = tx.query(&insert_album_stmt, &[&album.mbid, &album.name, &album.image_url, &existing_artist.as_ref().unwrap()]).await?;
            let album_id: i32 = row[0].get(0);
            existing_album = Some(album_id);
        }

        let track = imp.build_track(&rec, &rel);
        let insert_track_stmt = tx.prepare("INSERT INTO track (mbid, title, position, bit_rate, duration, file_location, album_id) VALUES ($1, $2, $3, $4, $5, $6, $7)").await?;
        tx.query(&insert_track_stmt, &[&track.mbid, &track.title, &(track.position as i32), &(track.bitrate as i32), &(track.duration as i32), &track.file_location, &existing_album.as_ref().unwrap()]).await?;

        tx.commit().await?;

        println!("Imported {} / {} by {}", track.title, rel.title, ac.artist.name);

        delay_for(Duration::from_secs(6)).await;
    }
    Ok(())
}

async fn existing_artist_album(album_mbid: &str, artist_mbid: &str, pool: deadpool_postgres::Pool)
    -> Result<(Option<i32>, Option<i32>), Box<dyn std::error::Error>>
{
    let client = pool.get().await?;
    // Fix this query
    let stmt = client.prepare("
        SELECT A.id, B.id
        FROM artist A
        LEFT OUTER JOIN album B
            ON B.mbid = $1
        WHERE A.mbid = $2
    ").await?;
    let rows = client.query(&stmt, &[&album_mbid, &artist_mbid]).await?;
    if rows.is_empty() {
        Ok((None, None))
    } else {
        Ok((rows[0].try_get::<'_, _, i32>(0).ok(),
            rows[0].try_get::<'_, _, i32>(1).ok()))
    }
}

fn create_pool() -> Result<deadpool_postgres::Pool, Box<dyn std::error::Error>> {
    let cfg = Config {
        user: env::var("POSTGRES_USER").ok(),
        password: env::var("POSTGRES_PW").ok(),
        dbname: env::var("POSTGRES_DB").ok(),
        options: None,
        application_name: None,
        ssl_mode: None,
        host: env::var("POSTGRES_HOST").ok(),
        hosts: None,
        port: env::var("POSTGRES_PORT").ok().map(|p| u16::from_str_radix(&p, 10).expect("Failed to convert port to integer")),
        ports: None,
        connect_timeout: None,
        keepalives: None,
        keepalives_idle: None,
        target_session_attrs: None,
        channel_binding: None,
        manager: None,
        pool: None,
    };
    Ok(cfg.create_pool(NoTls)?)
}
