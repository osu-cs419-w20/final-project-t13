import React from 'react'
import { ContextMenu, MenuItem, ContextMenuTrigger } from 'react-contextmenu';

import styles from './styles.css'

const TrackMenu = ({
  children,
  id,
  queue,
  addToPlaylist,
  openPlaylistModal,
}) => {
  return (
    <div className={styles.trackMenuWrapper}>
      <ContextMenuTrigger id={`${id}`} holdToDisplay={50}>
        {children}
      </ContextMenuTrigger>

      <ContextMenu id={`${id}`}>
        <MenuItem onClick={queue}>Queue Song</MenuItem>
        <MenuItem onClick={() => {
          addToPlaylist()
          openPlaylistModal()
        }}>Add to Playlist</MenuItem>
      </ContextMenu>
    </div>
  )
}

export default TrackMenu
