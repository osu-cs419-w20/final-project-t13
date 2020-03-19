export const RECEIVE_PLAYLIST = 'RECEIVE_PLAYLIST'
export const RECEIVE_PLAYLIST_TRACK = 'RECEIVE_PLAYLIST_TRACK'
export const RECEIVE_PLAYLISTS = 'RECEIVE_PLAYLISTS'

const receivePlaylist = json => ({
  type: RECEIVE_PLAYLIST,
  playlist: json,
  tracks: json.tracks,
})

const receivePlaylistTrack = (playlistId, json) => ({
  type: RECEIVE_PLAYLIST_TRACK,
  playlistId,
  track: json,
})

const fetchPlaylist = id => {
  return dispatch => {
    return fetch(`/api/playlists/${id}?relations=tracks`)
      .then(res => res.json())
      .then(json => dispatch(receivePlaylist(json)))
  }
}

const fetchPlaylistIfNeeded = id => {
  return (dispatch, getState) => {
    const state = getState()
    //if (!state.byId.playlists[id] || !state.byId.playlists[id].tracks) {
    return dispatch(fetchPlaylist(id))
    //} else {
      //return Promise.resolve()
    //}
  }
}

const receivePlaylists = json => ({
  type: RECEIVE_PLAYLISTS,
  playlists: json,
})

const fetchPlaylists = () => {
  return dispatch => {
    return fetch(`/api/playlists`)
      .then(res => res.json())
      .then(json => dispatch(receivePlaylists(json)))
  }
}

const fetchPlaylistsIfNeeded = () => {
  return (dispatch, getState) => {
    const state = getState()
    if (!state.byId.playlists.length) {
      return dispatch(fetchPlaylists())
    } else {
      return Promise.resolve()
    }
  }
}

const createPlaylist = (name) => {
  return dispatch => {
    return fetch('/api/playlists', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ name }),
    }).then(res => res.json())
      .then(json => dispatch(receivePlaylist(json)))
  }
}

const addTrackToPlaylist = (playlistId, trackId) => {
  return dispatch => {
    return fetch(`/api/playlists/${playlistId}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ trackId }),
    }).then(res => res.json())
      .then(json => dispatch(receivePlaylistTrack(playlistId, json)))
  }
}

export {
  fetchPlaylistIfNeeded,
  fetchPlaylistsIfNeeded,
  createPlaylist,
  addTrackToPlaylist,
}
