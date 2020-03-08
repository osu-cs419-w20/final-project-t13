#[derive(Debug)]
pub struct Artist {
    pub mbid: String,
    pub name: String,
    pub image_url: Option<String>,
}

#[derive(Debug)]
pub struct Album {
    pub mbid: String,
    pub name: String,
    pub image_url: Option<String>,
}

#[derive(Debug)]
pub struct Track {
    pub mbid: String,
    pub position: u16,
    pub title: String,
    pub bitrate: i64,
    pub duration: i64,
}
