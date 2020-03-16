import React, { useEffect } from 'react'

import { useDispatch, useSelector } from 'react-redux';
import { getArtists, getArtistsPagination } from '../reducers/artists'
import { fetchArtistsIfNeeded } from '../actions/artists'

import ArtistsPage from '../pages/artists'
import GridPage from '../components/grid-page'

const ArtistsRoute = (props) => {
  const { location } = props
  const query = new URLSearchParams(location.search)

  let page = query.get('page')
  if (page) {
    page = parseInt(page, 10)
  } else {
    page = 1
  }

  const artists = useSelector(getArtists)
  const pagination = useSelector(getArtistsPagination)
  const dispatch = useDispatch()
  useEffect(() => {
    document.title = `Artists - Page ${pagination.currentPage}`
    dispatch(fetchArtistsIfNeeded(page))
  }, [page, pagination.currentPage])

  return (
    <GridPage
      {...props}
      pageTitle="Artists"
      wrapperPageTitle={`Viewing Artists: ${pagination.currentPage} / ${pagination.totalPages}`}
    >
      <ArtistsPage
        artists={artists}
        pagination={pagination}
      />
    </GridPage>
  )
}

export default ArtistsRoute
