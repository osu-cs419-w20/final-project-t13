import React, { useEffect } from 'react'
import { useDispatch, useSelector } from 'react-redux';
import { useParams } from 'react-router-dom'

import { getArtist } from '../reducers/artists'

import { fetchArtistIfNeeded } from '../actions/artists'

import Page from '../components/page'
import ArtistPage from '../pages/artist'
import NotFoundPage from '../pages/not-found'

import { playAlbum } from '../actions/music-player'

const ArtistRoute = (props) => {
  const { artistId } = useParams()
  const id = parseInt(artistId, 10)

  if (isNaN(id)) {
    return <NotFoundPage {...props} />
  }

  const artistWithAlbums = useSelector(getArtist(id, ['albums']))
  const { albums, ...artist } = artistWithAlbums || {}

  const dispatch = useDispatch()

  useEffect(() => {
    dispatch(fetchArtistIfNeeded(id))
  }, [id])

  useEffect(() => {
    document.title = artist.name
  }, [artist.name])

  return (
    <Page {...props} pageTitle={`Viewing Artist: ${artist ? artist.name : ''}`}>
      <ArtistPage
        artist={artist}
        albums={albums}
        playAlbum={id => dispatch(playAlbum(id))}
      />
    </Page>
  )
}

export default ArtistRoute
