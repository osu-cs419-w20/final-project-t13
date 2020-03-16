export const RECEIVE_ALBUM = 'RECEIVE_ALBUM'
export const RECEIVE_ALBUMS = 'RECEIVE_ALBUMS'

const receiveAlbum = (json) => {
  return {
    type: RECEIVE_ALBUM,
    album: json,
    artist: json.artist,
    tracks: json.tracks,
  }
}

const fetchAlbum = (id) => {
  return dispatch => {
    return fetch(`/api/albums/${id}?relations=artist,tracks`)
      .then(res => res.json())
      .then(json => dispatch(receiveAlbum(json)))
  }
}

const shouldFetchAlbum = (state, id) => {
  return !state.byId.albums.hasOwnProperty(id) || !state.byId.albums[id].tracks
}

const fetchAlbumIfNeeded = (id) => {
  return (dispatch, getState) => {
    if (shouldFetchAlbum(getState(), id)) {
      return dispatch(fetchAlbum(id))
    } else {
      return Promise.resolve()
    }
  }
}

const receiveAlbums = (json) => {
  return {
    type: RECEIVE_ALBUMS,
    albums: json.data,
    artists: json.data.flatMap(a => a.artist ? [a.artist] : []),
    page: json.page,
    totalPages: json.total_pages,
  }
}

const fetchAlbums = (page) => {
  return dispatch => {
    return fetch(`/api/albums?relations=artist&page=${page}&limit=16`)
      .then(res => res.json())
      .then(json => dispatch(receiveAlbums(json)))
  }
}

const shouldFetchAlbums = (state, page) => {
  return !state.albums.hasLoaded || state.albums.page !== page
}

const fetchAlbumsIfNeeded = (page = 1) => {
  return (dispatch, getState) => {
    if (shouldFetchAlbums(getState(), page)) {
      return dispatch(fetchAlbums(page))
    } else {
      return Promise.resolve()
    }
  }
}

export { fetchAlbumIfNeeded, fetchAlbumsIfNeeded }
