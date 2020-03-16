import {
  NEXT_TRACK,
  PAUSE_MUSIC,
  PLAY_MUSIC,
  PREV_TRACK,
  QUEUE_TRACKS,
  SET_CURRENT_TIME,
  SET_CURRENT_TRACK,
  SET_DURATION,
  STOP_MUSIC,
} from '../actions/music-player'
import { getAlbum } from './albums'
import { getArtist } from './artists'

const musicPlayer = (
  state = {
    prevStack: [],
    nextQueue: [],
    currentSong: null,

    currentTrack: null,
    prev: [],
    next: [],

    playing: false,
    currentTime: 0,
    duration: 0
  },
  action
) => {
  switch (action.type) {
    case NEXT_TRACK:
      const [nextId, ...remaining] = state.next
      if (!nextId) { return state }
      return {
        ...state,
        currentTrack: nextId,
        prev: state.currentTrack ? [...state.prev, state.currentTrack] : [],
        next: remaining || [],
      }
    case PAUSE_MUSIC:
      return { ...state, playing: false }
    case PLAY_MUSIC:
      if (!state.currentTrack && state.next.length) {
        const [top, ...next] = state.next
        return {
          ...state,
          currentTrack: top,
          next: next || [],
          playing: true
        }
      }
      return { ...state, playing: true }
    case PREV_TRACK:
      const prevTrackCount = state.prev.length
      if (prevTrackCount < 1) { return state }

      return {
        ...state,
        prev: state.prev.filter((_, i) => i !== prevTrackCount - 1),
        next: [state.currentTrack, ...state.next],
        currentTrack: state.prev[prevTrackCount - 1],
      }
    case QUEUE_TRACKS:
      return {
        ...state,
        next: [...state.next, ...action.tracks],
      }
    case SET_CURRENT_TIME:
      return { ...state, currentTime: action.secs }
    case SET_DURATION:
      return { ...state, duration: action.secs }
    case SET_CURRENT_TRACK:
      return {
        ...state,
        currentTrack: action.id,
        prev: [],
        next: action.next,
      }
    case STOP_MUSIC:
      return {
        ...state,
        prev: [],
        next: [],
        currentTrack: null,
        playing: false,
        currentTime: 0,
        duration: 0,
      }
    default:
      return state
  }
}

const getPlayerState = (state) => {
  return {
    currentTime: state.musicPlayer.currentTime,
    duration: state.musicPlayer.duration,
    playing: state.musicPlayer.playing,
  }
}

const getQueue = (state) => {
  const {
    prev,
    next,
    currentTrack,
  } = state.musicPlayer
  const buildQueueTrack = tid => {
    var track = state.byId.tracks[tid]
    if (!track) {
      var { position, ...track } = state.byId.playlistTracks[tid] || {}
      return track
    }

    const album = getAlbum(track.album_id)(state)
    const artist = getArtist(album.artist_id)(state)

    return {
      id: track.id,
      title: track.title,
      duration: track.duration,
      artistId: artist.id,
      artistName: artist.name,
      albumId: album.id,
      albumTitle: album.title,
      albumImage: album.image_url,
    }
  }

  return {
    prev: prev.map(buildQueueTrack),
    next: next.map(buildQueueTrack),
    current: currentTrack ? buildQueueTrack(currentTrack) : null,
  }
}

const getCurrentTrack = (state) => {
  if (!state.musicPlayer.currentTrack) { return null }
  const track = state.byId.tracks[state.musicPlayer.currentTrack]
  if (!track) { return null }
  const album = state.byId.albums[track.album_id]

  return { album, track }
}

export default musicPlayer
export {
  getQueue,
  getPlayerState,
  getCurrentTrack,
}
