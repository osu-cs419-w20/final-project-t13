import React from 'react'

import CoverImage from '../../components/cover-image'
import ArtGrid from '../../components/art-grid'

import ArtistInfo from './artist-info'
import SingleAlbum from './single-album'

import styles from './styles.css'

const ArtistPage = ({ artist, albums, playAlbum }) => {
  return (
    <div className={styles.wrapper}>
      {artist && albums &&
        <>
          <div className={styles.leftColumn}>
            <CoverImage image={artist.image_url} imageClass={styles.artistImage} />

            <ArtistInfo albumCount={albums.length} name={artist.name} />
          </div>

          <div className={styles.rightColumn}>
            <h3 className={styles.rightColumnLabel}>Albums from {artist.name}</h3>

            <ArtGrid perRow={2} className={styles.albumsGrid}>
              {albums.map(a => <SingleAlbum key={a.id} album={a} play={() => playAlbum(a.id)} className={styles.album}/>)}
            </ArtGrid>
          </div>
        </>}
    </div>
  )
}

export default ArtistPage
