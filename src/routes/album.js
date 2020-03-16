import React, { useEffect } from 'react'

import { useParams } from 'react-router-dom'
import { useDispatch, useSelector } from 'react-redux';
import { getAlbum, getAlbumDuration, getArtistForAlbum, getTracksForAlbum } from '../reducers/albums'
import { fetchAlbumIfNeeded } from '../actions/albums'
import { getPlaylists } from '../reducers/by-id'
import { addTrackToPlaylist } from '../actions/playlists'

import AddToPlaylistModal from '../components/add-to-playlist-modal'
import Page from '../components/page'
import AlbumPage from '../pages/album'

import { playTrack, queueAlbum, queueTracks } from '../actions/music-player'

const AlbumRoute = (props) => {
  const { albumId } = useParams()
  const id = parseInt(albumId, 10)

  const album = useSelector(getAlbum(id))
  const artist = useSelector(getArtistForAlbum(id))
  const tracks = useSelector(getTracksForAlbum(id))
  const duration = useSelector(getAlbumDuration(id))

  const playlists = useSelector(getPlaylists)

  const dispatch = useDispatch()

  const [modalOpen, setModalOpen] = React.useState(false)
  const [modalTrackId, setModalTrackId] = React.useState(null)

  useEffect(() => {
    dispatch(fetchAlbumIfNeeded(id))
  }, [id])

  useEffect(() => {
    if (album) {
      document.title = album.title
    }
  }, [album])

  return (
    <Page {...props} pageTitle={`Viewing Album: ${album ? album.title : ''}`}>
      <AddToPlaylistModal
        isOpen={modalOpen}
        close={() => setModalOpen(false)}
        playlists={playlists}
        addToPlaylist={playlistId => dispatch(addTrackToPlaylist(playlistId, modalTrackId))}
      />

      <AlbumPage
        album={album}
        albumDuration={duration}
        artist={artist}
        tracks={tracks}
        play={(track, next) => dispatch(playTrack(track, next))}
        openPlaylistModal={() => setModalOpen(true)}
        addToPlaylist={setModalTrackId}
        queueAlbum={() => {
          if (album) {
            dispatch(queueAlbum(album.id))
          }
        }}
        queueTrack={id => dispatch(queueTracks([id]))}
      />
    </Page>
  )
}

export default AlbumRoute
