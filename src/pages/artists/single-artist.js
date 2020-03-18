import React from 'react'
import { Link } from 'react-router-dom'

import CoverImage from '../../components/cover-image'

import styles from './styles.css'

const SingleArtist = ({ artist, className = '' }) => {
  return (
    <li className={className}>
      <div>
        <CoverImage
          link={`/artists/${artist.id}`}
          image={artist.image_url}
          imageClass={styles.artistImage}
        />

        <div className={styles.artistInfo}>
            <Link
              to={`/artists/${artist.id}`}
              className={`${styles.link} ${styles.artistLink}`}
            >{artist.name}</Link>
        </div>
      </div>
    </li>
  )
}

export default SingleArtist
