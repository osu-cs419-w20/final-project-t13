import React from 'react'
import { NavLink } from 'react-router-dom'

import PlaylistIcon from '../../icons/playlist.svg'
import styles from './styles.css'

const PlaylistsMenu = ({ playlists }) => {
  return (
    <nav className={styles.playlistMenu}>
      {playlists.map(p => (
        <NavLink exact key={p.id} to={`/playlists/${p.id}`} className={styles.playlistMenuLink}>
          <PlaylistIcon className={styles.playlistIcon} />
          <span className={styles.playlistName}>{p.name}</span>
        </NavLink>
      ))}
    </nav>
  )
}

export default PlaylistsMenu
