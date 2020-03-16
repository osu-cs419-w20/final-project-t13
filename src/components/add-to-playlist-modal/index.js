import React from 'react'
import Modal from 'react-modal'

import styles from './styles.css'

const AddToPlaylistModal = ({
  isOpen,
  close,
  playlists,
  addToPlaylist,
}) => {
  const handleClick = id => {
    return (e) => {
      e.preventDefault()
      addToPlaylist(id)
      close()
    }
  }

  return (
    <Modal
      isOpen={isOpen}
      onRequestClose={close}
      className={styles.modal}
      overlayClassName={styles.modalOverlay}
      shouldCloseOnOverlayClick={true}
    >
      <header className={styles.header}>
        <h4 className={styles.title}>Add to Playlist</h4>
        <button onClick={close} className={styles.closeButton}>x</button>
      </header>

      <ul className={styles.playlists}>
        {playlists.map(p => (
          <li key={p.id} className={styles.playlist}>
            <a className={styles.playlistLink} onClick={handleClick(p.id)} href="#">{p.name}</a>
          </li>
        ))}
      </ul>
    </Modal>
  )
}

export default AddToPlaylistModal
