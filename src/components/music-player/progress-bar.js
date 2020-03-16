import React from 'react'

import styles from './styles.css'

const ProgressBar = ({ currentTime, duration }) => {
  const pct = (currentTime === 0 || duration === 0) ? 0 : (currentTime / duration) * 100
  const bar = { width: `${pct}%` }
  return (
    <div className={styles.progressBar}>
      <div className={styles.progressBarInner} style={bar}></div>
    </div>
  )
}

export default ProgressBar
