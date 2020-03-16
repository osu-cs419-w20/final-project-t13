import React from 'react'

import ArtGrid from '../../components/art-grid'
import Pagination from '../../components/pagination'
import SingleAlbum from './single-album'

import styles from './styles.css'

const AlbumsPage = ({ albums, pagination, playAlbum }) => {
  const { currentPage, totalPages } = pagination

  return (
    <div className={styles.wrapper}>
      <ArtGrid perRow={4} className={styles.albumsGrid}>
        {albums.map(a => <SingleAlbum key={a.id} album={a} play={() => playAlbum(a.id)} className={styles.album} />)}
      </ArtGrid>

      <Pagination currentPage={currentPage} totalPages={totalPages} path="/albums" />
    </div>
  )
}

export default AlbumsPage
