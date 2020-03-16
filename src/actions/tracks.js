export const RECEIVE_TRACK = 'RECEIVE_TRACK'

const receiveTrack = json => ({
  type: RECEIVE_TRACK,
  track: json,
})

const fetchTrack = id => {
  return dispatch => {
    return fetch(`/api/tracks/${id}`)
      .then(res => res.json())
      .then(json => dispatch(receiveTrack(json)))
  }
}

const retrieveTrack = id => {
  return (dispatch, getState) => {
    if (!getState().byId.tracks[id]) {
      return dispatch(fetchTrack(id))
    } else {
      return Promise.resolve()
    }
  }
}

export { retrieveTrack }
