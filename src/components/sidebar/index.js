import React from 'react'
import { NavLink } from 'react-router-dom'
import { useDispatch, useSelector } from 'react-redux'

import { createPlaylist, fetchPlaylistsIfNeeded } from '../../actions/playlists'
import { getPlaylists } from '../../reducers/by-id'

import CreatePlaylistForm from './create-playlist-form'
import PlaylistsMenu from './playlists-menu'

import styles from './styles.css'

const Sidebar = () => {
  const playlists = useSelector(getPlaylists)
  const dispatch = useDispatch()

  React.useEffect(() => {
    dispatch(fetchPlaylistsIfNeeded())
  }, [])

  return (
    <div className={styles.sidebar}>
      <section className={styles.sidebarSection}>
        <h3 className={styles.sidebarSectionTitle}>Menu</h3>

        <nav className={styles.globalNavMenu}>
          <NavLink exact to="/" className={styles.navLink} >Home</NavLink>
          <NavLink to="/albums" className={styles.navLink}>Albums</NavLink>
          <NavLink to="/artists" className={styles.navLink}>Artists</NavLink>
        </nav>
      </section>

      <section className={styles.sidebarSection}>
        <h3 className={styles.sidebarSectionTitle}>Playlists</h3>

        <PlaylistsMenu playlists={playlists} />
      </section>

      <section className={styles.sidebarSection}>
        <h3 className={styles.sidebarSectionTitle}>Create Playlist</h3>

        <CreatePlaylistForm create={name => dispatch(createPlaylist(name))} />
      </section>
    </div>
  )
}

export default Sidebar
