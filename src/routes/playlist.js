import React, { useEffect } from 'react'

import { useParams } from 'react-router-dom'
import { useDispatch, useSelector } from 'react-redux';
import { getPlaylist } from '../reducers/by-id'
import { fetchPlaylistIfNeeded } from '../actions/playlists'
import { getPlaylists } from '../reducers/by-id'
import { addTrackToPlaylist } from '../actions/playlists'

import AddToPlaylistModal from '../components/add-to-playlist-modal'
import Page from '../components/page'
import PlaylistPage from '../pages/playlist'
import NotFoundPage from '../pages/not-found'

import { playTrack, queueTracks } from '../actions/music-player'

const PlaylistRoute = (props) => {
  const { playlistId } = useParams()
  const id = parseInt(playlistId, 10)

  if (isNaN(id)) {
    return <NotFoundPage {...props} />
  }

  const playlist = useSelector(getPlaylist(id))
  const playlists = useSelector(getPlaylists)

  const dispatch = useDispatch()

  const [modalOpen, setModalOpen] = React.useState(false)
  const [modalTrackId, setModalTrackId] = React.useState(null)

  useEffect(() => {
    dispatch(fetchPlaylistIfNeeded(id))
  }, [id])

  useEffect(() => {
    if (playlist) {
      document.title = `Playlist: ${playlist.name}`
    }
  }, [playlist])

  return (
    <Page {...props} pageTitle={`Viewing Playlist: ${playlist ? playlist.name : ''}`}>
      <AddToPlaylistModal
        isOpen={modalOpen}
        close={() => setModalOpen(false)}
        playlists={playlists}
        addToPlaylist={playlistId => dispatch(addTrackToPlaylist(playlistId, modalTrackId))}
      />

      <PlaylistPage
        playlist={playlist}
        play={(track, next) => dispatch(playTrack(track.id, next))}
        queue={ids => dispatch(queueTracks(Array.isArray(ids) ? ids : [ids]))}
        addToPlaylist={setModalTrackId}
        openPlaylistModal={() => setModalOpen(true)}
      />
    </Page>
  )
}

export default PlaylistRoute
