import React from 'react'
import { Link } from 'react-router-dom'

import CoverImage from '../cover-image'

import { displayTime } from '../../utils'

import styles from './styles.css'

const QueueItem = ({
  track,
  isPrev = false,
  isCurrent = false,
}) => {

  let itemStateClassName = ''
  
  if (isPrev) { itemStateClassName = styles.queuePrevItem }
  if (isCurrent) { itemStateClassName = styles.queueCurrentItem }

  return (
    <li className={`${styles.queueItem} ${itemStateClassName}`}>
      <CoverImage link={`/albums/${track.albumId}`} image={track.albumImage} imageClass={styles.queueImage} />

      <div className={styles.queueTrackId}>
        <p className={styles.queueTrackTitle}>{track.title}</p>
        <Link to={`/albums/${track.albumId}`} className={styles.queueTrackAlbum}>{track.albumTitle}</Link>
      </div>

      <span className={styles.queueTrackDuration}>{displayTime(track.duration)}</span>
    </li>
  )
}

export default QueueItem
