use rspotify::client::Spotify;
use rspotify::oauth2::SpotifyClientCredentials;
use rspotify::model::album::{FullAlbum, SimplifiedAlbum};
use rspotify::model::artist::FullArtist;

#[derive(Clone)]
pub struct Client {
    client: Spotify,
}

impl Client {
    pub fn new() -> Client {
        let client_credentials = SpotifyClientCredentials::default().build();
        let client = Spotify::default()
            .client_credentials_manager(client_credentials)
            .build();

        Client {
            client
        }
    }

    pub async fn get_artist(&self, id: &str) -> Result<FullArtist, Box<dyn std::error::Error>> {
        self.client.artist(id).await.map_err(|e| e.into())
    }

    pub async fn get_album(&self, id: &str) -> Result<FullAlbum, Box<dyn std::error::Error>> {
        self.client.album(id).await.map_err(|e| e.into())
    }

    pub async fn search_album(&self, album: &str, artist: &str) -> Result<Vec<SimplifiedAlbum>, Box<dyn std::error::Error>> {
        let q = format!("album:{} artist:{}", album, artist);
        Ok(self.client.search_album(&q, 10, 0, None).await?.albums.items)
    }

    pub async fn search_artist(&self, artist: &str) -> Result<Vec<FullArtist>, Box<dyn std::error::Error>> {
        let q = format!("artist:{}", artist);
        Ok(self.client.search_artist(&q, 10, 0, None).await?.artists.items)
    }
}
