import { RECEIVE_ALBUMS } from '../actions/albums'

const albums = (state = {
  ids: [],
  page: 1,
  totalPages: 1,
}, action) => {
  switch (action.type) {
    case RECEIVE_ALBUMS:
      return {
        ...state,
        ids: action.albums.map(a => a.id),
        page: action.page,
        totalPages: action.totalPages,
      }
    default:
      return state
  }
}

const getAlbum = (id) => {
  return (state) => state.byId.albums[id]
}
const getAlbumsPagination = (state) => {
  return {
    currentPage: state.albums.page,
    totalPages: state.albums.totalPages,
  }
}
const getArtistForAlbum = (id) => {
  return (state) => {
    const album = state.byId.albums[id]
    if (!album) { return {} }
    return state.byId.artists[album.artist_id]
  }
}
const getAlbumsWithArtist = (state) => {
  return state.albums.ids.map(id => {
    const album = state.byId.albums[id]
    const artist = state.byId.artists[album.artist_id]
    return { ...album, artist }
  })
}
const getTracksForAlbum = (id) => {
  return (state) => {
    const album = state.byId.albums[id]
    if (album && album.tracks) {
      return album.tracks.map(tid => state.byId.tracks[tid]).sort((a, b) => a.position - b.position)
    } else {
      return []
    }
  }
}
const getAlbumDuration = (id) => {
  return (state) => {
    const album = state.byId.albums[id]
    if (!album || !album.tracks) { return 0 }
    return album.tracks.reduce((acc, tid) => acc + state.byId.tracks[tid].duration, 0)
  }
}

export default albums
export {
  getAlbum,
  getAlbumDuration,
  getAlbumsPagination,
  getAlbumsWithArtist,
  getArtistForAlbum,
  getTracksForAlbum
}
