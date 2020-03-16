import { fetchAlbumIfNeeded } from './albums'
import { retrieveTrack } from './tracks'
import { getTracksForAlbum } from '../reducers/albums'

export const PAUSE_MUSIC = 'PAUSE_MUSIC'
export const PLAY_MUSIC = 'PLAY_MUSIC'
export const SET_CURRENT_TIME = 'SET_CURRENT_TIME'
export const SET_DURATION = 'SET_DURATION'
export const STOP_MUSIC = 'STOP_MUSIC'
export const PLAY_ALBUM = 'PLAY_ALBUM'
export const QUEUE_TRACKS = 'QUEUE_TRACKS'
export const SET_CURRENT_TRACK = 'SET_CURRENT_TRACK'
export const NEXT_TRACK = 'NEXT_TRACK'
export const PREV_TRACK = 'PREV_TRACK'

const pauseMusic = () => {
  return { type: PAUSE_MUSIC }
}

const playMusic = () => {
  return { type: PLAY_MUSIC }
}

const stopMusic = () => ({ type: STOP_MUSIC })

const setCurrentTime = (secs) => {
  return {
    type: SET_CURRENT_TIME,
    secs
  }
}

const setDuration = (secs) => {
  return {
    type: SET_DURATION,
    secs
  }
}

const setCurrentTrack = (id, next = []) => ({
  type: SET_CURRENT_TRACK,
  id,
  next,
})

const playPlaylistTrack = (id, pos) => {
  return (dispatch, getState) => {
    return dispatch(retrieveTrack(id))
      .then(() => {
        const state = getState()
        const next = playlist.tracks.flatMap(t => t.position > pos ? [t.id] : [])
        dispatch(setCurrentTrack(id, next))
      })
  }
}

const playTrack = (id, next = []) => {
  return (dispatch, getState) => {
    return dispatch(retrieveTrack(id))
      .then(() => dispatch(setCurrentTrack(id, next)))
  }
}

const queueTracks = (tracks) => {
  return {
    type: QUEUE_TRACKS,
    tracks,
  }
}

const playAlbum = (id) => {
  return (dispatch, getState) => {
    return dispatch(fetchAlbumIfNeeded(id)).then(() => {
      const tracks = getTracksForAlbum(id)(getState())
      const [first, ...remaining] = tracks
      if (!first) {
        return Promise.resolve()
      }

      const next = remaining ? remaining.flatMap(t => t.position > 1 ? [t.id] : []) : []
      dispatch(playTrack(first.id, next))
    })
  }
}

const queueAlbum = (id) => {
  return (dispatch, getState) => {
    return dispatch(fetchAlbumIfNeeded(id)).then(() => {
      const tracks = getTracksForAlbum(id)(getState()).map(t => t.id)
      dispatch(queueTracks(tracks))
    })
  }
}

const nextTrack = () => ({ type: NEXT_TRACK })
const prevTrack = () => ({ type: PREV_TRACK })

export {
  pauseMusic,
  playMusic,
  stopMusic,
  setCurrentTime,
  setDuration,
  playAlbum,
  queueAlbum,
  playTrack,
  playPlaylistTrack,
  queueTracks,
  nextTrack,
  prevTrack,
}
