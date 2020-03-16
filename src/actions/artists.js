export const RECEIVE_ARTISTS = 'RECEIVE_ARTISTS'
export const RECEIVE_ARTIST = 'RECEIVE_ARTIST'

const receiveArtist = json => {
  return {
    type: RECEIVE_ARTIST,
    artist: json,
    albums: json.albums,
  }
}

const fetchArtist = (id) => {
  return dispatch => {
    return fetch(`/api/artists/${id}?relations=albums`)
      .then(res => res.json())
      .then(json => dispatch(receiveArtist(json)))
  }
}

const fetchArtistIfNeeded = (id) => {
  return (dispatch, getState) => {
    const state = getState()
    if (!state.byId.artists.hasOwnProperty(id) || !state.byId.artists[id].albums) {
      return dispatch(fetchArtist(id))
    }
  }
}

const receiveArtists = (json) => {
  return {
    type: RECEIVE_ARTISTS,
    artists: json.data,
    page: json.page,
    totalPages: json.total_pages,
  }
}

const fetchArtists = (page) => {
  return dispatch => {
    return fetch(`/api/artists?page=${page}&limit=15`)
      .then(res => res.json())
      .then(json => dispatch(receiveArtists(json)))
  }
}

const shouldFetchArtists = (state, page) => {
  return !state.artists.hasLoaded || state.artists.page !== page
}

const fetchArtistsIfNeeded = (page = 1) => {
  return (dispatch, getState) => {
    if (shouldFetchArtists(getState(), page)) {
      return dispatch(fetchArtists(page))
    }
  }
}

export { fetchArtistIfNeeded, fetchArtistsIfNeeded }
