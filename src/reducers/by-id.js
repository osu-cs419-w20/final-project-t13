import {
  RECEIVE_ALBUM,
  RECEIVE_ALBUMS
} from '../actions/albums'

import {
  RECEIVE_ARTIST,
  RECEIVE_ARTISTS,
} from '../actions/artists'

import {
  RECEIVE_PLAYLIST,
  RECEIVE_PLAYLIST_TRACK,
  RECEIVE_PLAYLISTS,
} from '../actions/playlists'

import { RECEIVE_TRACK } from '../actions/tracks'

const byId = (state = {
  albums: {},
  artists: {},
  playlists: {},
  playlistTracks: {},
  tracks: {},
}, action) => {
  switch (action.type) {
    case RECEIVE_ALBUM:
      var { artist, tracks, ...album } = action.album
      var currentArtist = state.artists[artist.id] || {}
      let c = {
        ...state,
        albums: {
          ...state.albums,
          [action.album.id]: {
            ...album,
            tracks: tracks.map(t => t.id)
          }
        },
        artists: { ...state.artists, [artist.id]: { ...currentArtist, ...artist } },
        tracks: tracks.reduce((acc, t) => ({ ...acc, [t.id]: t }), state.tracks),
      }
      return c
    case RECEIVE_ALBUMS:
      return {
        ...state,
        albums: action.albums.reduce((acc, a) => ({ ...acc, [a.id]: a }), state.albums),
        artists: action.albums
          .filter(a => a.artist)
          .map(a => a.artist)
          .reduce((acc, a) => ({ ...acc, [a.id]: a}), state.artists)
      }
    case RECEIVE_ARTIST:
      const { albums, ...artistWithoutAlbums } = action.artist
      const normalizedArtist = { ...artistWithoutAlbums, albums: albums.map(a => a.id) }
      return {
        ...state,
        albums: albums.reduce((acc, a) => ({ ...acc, [a.id]: a }), state.albums),
        artists: { ...state.artists, [normalizedArtist.id]: normalizedArtist }
      }
    case RECEIVE_ARTISTS:
      return {
        ...state,
        artists: action.artists.reduce((acc, a) => ({ ...acc, [a.id]: a }), state.artists),
      }
    case RECEIVE_PLAYLIST:
      var { tracks, ...playlist } = action.playlist
      if (!tracks) { tracks = [] }
      return {
        ...state,
        playlists: {
          ...state.playlists,
          [action.playlist.id]: { ...playlist, tracks: tracks.map(t => t.id) },
        },
        playlistTracks: tracks.reduce((acc, t) => ({ ...acc, [t.id]: t }), state.playlistTracks),
      }
    case RECEIVE_PLAYLIST_TRACK:
      var playlist = state.playlists[action.playlistId]
      var tracks

      if (playlist && playlist.tracks) {
        if (!playlist.tracks.includes(action.track.id)) {
          tracks = [...playlist.tracks, action.track.id]
        }
      } else {
        tracks = [action.track.id]
      }

      return {
        ...state,
        playlistTracks: {
          ...state.playlistTracks,
          [action.track.id]: action.track,
        },
        playlists: playlist ? { ...state.playlists, [playlist.id]: { ...playlist, tracks } } : state.playlists,
      }
    case RECEIVE_PLAYLISTS:
      return {
        ...state,
        playlists: action.playlists.reduce((acc, p) => ({ ...acc, [p.id]: p }), state.playlists),
      }
    case RECEIVE_TRACK:
      var { album, artist, ...track } = action.track
      const currAlbum = state.albums[album.id] || {}
      const currArtist = state.artists[artist.id] || {}
      return {
        ...state,
        tracks: { ...state.tracks, [track.id]: track },
        artists: { ...state.artists, [artist.id]: { ...currArtist, ...artist } },
        albums: { ...state.albums, [album.id]: { ...currAlbum, ...album } },
      }
    default:
      return state
  }
}

const getPlaylist = id => {
  return (state) => {
    const playlist = state.byId.playlists[id]
    if (!playlist) { return null }
    const tracks = playlist.tracks ? playlist.tracks.map(tid => state.byId.playlistTracks[tid]) : []
    return { ...playlist, tracks }
  }
}
const getPlaylists = (state) => Object.values(state.byId.playlists)

export default byId
export { getPlaylist, getPlaylists }
