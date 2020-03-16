import React from 'react'

import styles from './styles.css'

const Actions = ({ play, queue }) => {
  return (
    <div className={styles.actions}>
      <button onClick={play} className={styles.actionButton}>Play</button>
      <button onClick={queue} className={styles.actionButton}>Queue</button>
    </div>
  )
}

export default Actions
