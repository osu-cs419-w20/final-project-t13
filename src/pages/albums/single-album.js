import React from 'react'
import { Link } from 'react-router-dom'

import CoverImage from '../../components/cover-image'

import styles from './styles.css'

const SingleAlbum = ({ album, className = '', play }) => {
  return (
    <li className={className}>
      <div>
        <CoverImage link={`/albums/${album.id}`} image={album.image_url} play={play} />

        <div className={styles.albumInfo}>
            <Link
              to={`/albums/${album.id}`}
              className={`${styles.link} ${styles.trackLink}`}
            >{album.title}</Link>

            <Link
              to={`/artists/${album.artist.id}`}
              className={`${styles.link} ${styles.artistLink}`}
            >{album.artist.name}</Link>
        </div>
      </div>
    </li>
  )
}

export default SingleAlbum
