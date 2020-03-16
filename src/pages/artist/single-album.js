import React from 'react'
import { Link } from 'react-router-dom'

import CoverImage from '../../components/cover-image'

import styles from './styles.css'

const SingleAlbum = ({ album, className = '', play }) => {
  return (
    <li className={className}>
      <div>
        <CoverImage link={`/albums/${album.id}`} image={album.image_url} play={play} imageClass={styles.albumImage} />

        <div className={styles.albumInfo}>
            <Link
              to={`/albums/${album.id}`}
              className={`${styles.link} ${styles.albumLink}`}
            >{album.title}</Link>
        </div>
      </div>
    </li>
  )
}

export default SingleAlbum
