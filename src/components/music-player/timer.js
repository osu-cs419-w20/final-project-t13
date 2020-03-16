import React from 'react'

import { displayTime } from '../../utils'

import styles from './styles.css'

const Timer = (props) => {
  const { currentTime, duration } = props
  return (
    <div className={styles.timer}>
      <span className={styles.currentTime}>{displayTime(currentTime)}</span>
      /
      <span className={styles.timerDuration}>{displayTime(duration)}</span>
    </div>
  )
}

export default Timer
