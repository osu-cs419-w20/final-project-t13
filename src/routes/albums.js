import React, { useEffect } from 'react'

import { useDispatch, useSelector } from 'react-redux';
import { getAlbumsPagination, getAlbumsWithArtist } from '../reducers/albums'
import { fetchAlbumsIfNeeded } from '../actions/albums'
import { playAlbum } from '../actions/music-player'

import GridPage from '../components/grid-page'

import AlbumsPage from '../pages/albums'

const AlbumsRoute = (props) => {
  const { location } = props
  const query = new URLSearchParams(location.search)

  let page = query.get('page')
  if (page) {
    page = parseInt(page, 10)
    if (isNaN(page)) { page = 1 }
  } else {
    page = 1
  }

  const pagination = useSelector(getAlbumsPagination)
  const albums = useSelector(getAlbumsWithArtist)
  const dispatch = useDispatch()

  useEffect(() => {
    document.title = `Albums - Page ${pagination.currentPage}`
    dispatch(fetchAlbumsIfNeeded(page))
  }, [page, pagination.currentPage])

  return (
    <GridPage
      {...props}
      pageTitle="Albums"
      wrapperPageTitle={`Viewing Albums: ${pagination.currentPage} / ${pagination.totalPages}`}
    >
      <AlbumsPage
        albums={albums}
        pagination={pagination}
        playAlbum={id => dispatch(playAlbum(id))}
      />
    </GridPage>
  )
}

export default AlbumsRoute

