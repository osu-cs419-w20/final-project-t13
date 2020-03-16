import React from 'react'

import styles from './styles.css'

const ArtistInfo = ({ albumCount, name }) => {
  return (
    <div className={styles.artistInfo}>
      <h1 className={styles.artistName}>{name}</h1>

      <div className={styles.metadata}>
        <span className={styles.metadataItem}>{albumCount} albums</span>
      </div>
    </div>
  )
}

export default ArtistInfo
