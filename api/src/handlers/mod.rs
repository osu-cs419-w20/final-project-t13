pub mod albums;
pub mod artists;
pub mod playlists;
pub mod tracks;

#[derive(Serialize)]
struct PaginatedResponse<T> {
    page: i64,
    count: i64,
    total_pages: i64,
    data: Vec<T>,
}

