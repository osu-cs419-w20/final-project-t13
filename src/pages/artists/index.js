import React from 'react'

import ArtGrid from '../../components/art-grid'
import Pagination from '../../components/pagination'
import SingleArtist from './single-artist'

import styles from './styles.css'

const ArtistsPage = ({ artists, pagination }) => {
  const { currentPage, totalPages } = pagination

  return (
    <div className={styles.wrapper}>
      <ArtGrid perRow={4} className={styles.artistsGrid}>
        {artists.map(a => <SingleArtist key={a.id} artist={a} className={styles.artist} />)}
      </ArtGrid>

      <Pagination currentPage={currentPage} totalPages={totalPages} path="/artists" />
    </div>
  )
}

export default ArtistsPage
