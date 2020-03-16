import React from 'react'

import { displayTime } from '../../utils'

import styles from './styles.css'

const Metadata = ({ duration, trackCount }) => {
  return (
    <div className={styles.metadata}>
      <span className={styles.metadataItem}>{trackCount} songs</span>
      <span className={styles.metadataItem}>{displayTime(duration)}</span>
    </div>
  )
}

export default Metadata
