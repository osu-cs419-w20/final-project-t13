import React from 'react'
import { Link } from 'react-router-dom'

import CoverImage from '../cover-image'

import MusicNoteIcon from '../../icons/music-note.svg'
import styles from './styles.css'

const CurrentSong = ({ album, song }) => {
  const image = album ? album.image_url : null
  const songTitle = song ? song.title : 'Not Playing'

  return (
    <div className={styles.currentSong}>
      {image && <CoverImage image={image} imageClass={styles.albumImage} />}
      {!image &&
        <MusicNoteIcon className={styles.notPlayingIcon} height={80} width={80} />}

      <div className={styles.currentSongInfo}>
        <span className={styles.songTitle}>{songTitle}</span>
        {album && <Link to={`/albums/${album.id}`} className={styles.albumLink}>{album.title}</Link>}
      </div>
    </div>
  )
}

export default React.memo(CurrentSong)
