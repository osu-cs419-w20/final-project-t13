import React from 'react'
import { Link } from 'react-router-dom'

import styles from './styles.css'

const Header = ({ title, artist }) => {
  return (
    <header className={styles.albumHeader}>
      <h1 className={styles.albumTitle}>{title}</h1>
      <div className={styles.headerMetadata}>
        <Link to={`/artists/${artist.id}`} className={styles.artistLink}>{artist.name}</Link>
      </div>
    </header>
  )
}

export default Header
