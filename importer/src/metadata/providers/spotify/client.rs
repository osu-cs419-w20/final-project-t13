use rspotify::client::Spotify;
use rspotify::oauth2::SpotifyClientCredentials;
use rspotify::model::album::FullAlbum;
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
}
