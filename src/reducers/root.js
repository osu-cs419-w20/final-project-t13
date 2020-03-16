import { combineReducers } from 'redux'

import albums from './albums'
import artists from './artists'
import byId from './by-id'
import musicPlayer from './music-player'

export default combineReducers({
  albums,
  artists,
  byId,
  musicPlayer,
})
