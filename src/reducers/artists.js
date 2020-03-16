import { RECEIVE_ARTISTS } from '../actions/artists'

const artists = (state = {
  ids: [],
  page: 1,
  totalPages: 1,
}, action) => {
  switch (action.type) {
    case RECEIVE_ARTISTS:
      return {
        ...state,
        ids: action.artists.map(a => a.id),
        page: action.page,
        totalPages: action.totalPages,
      }
    default:
      return state
  }
}

const getArtist = (id, relations = []) => {
  return (state) => {
    const artist = state.byId.artists[id]
    if (!artist) return null

    let albums = []
    if (relations.includes('albums') && artist.albums) {
      albums = artist.albums.map(aid => state.byId.albums[aid])
    }

    return { ...artist, albums }
  }
}
const getArtistsPagination = (state) => {
  return {
    currentPage: state.artists.page,
    totalPages: state.artists.totalPages,
  }
}
const getArtists = (state) => {
  return state.artists.ids.map(id => state.byId.artists[id])
}

export default artists
export { getArtist, getArtists, getArtistsPagination }
