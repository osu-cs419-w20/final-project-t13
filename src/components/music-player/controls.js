import React from 'react'

import PrevIcon from '../../icons/prev.svg'
import PauseIcon from '../../icons/pause.svg'
import PlayIcon from '../../icons/play.svg'
import NextIcon from '../../icons/next.svg'
import StopIcon from '../../icons/stop.svg'
import QueueIcon from '../../icons/queue.svg'

import styles from './styles.css'

const Controls = (props) => {
  const {
    isPlaying,
    pause,
    play,
    song,
    prevSong,
    nextSong,
    stop,
    toggleQueue,
    queue,
  } = props

  return (
    <div className={styles.controls}>
      <button onClick={prevSong} className={styles.controlButton} disabled={queue.prev.length < 1 || !isPlaying}>
        <PrevIcon className={styles.controlIcon} />
      </button>
      {isPlaying &&
        <button onClick={pause} className={`${styles.controlButton} ${styles.playButton}`}>
          <PauseIcon className={styles.controlIcon} />
        </button>}
      {!isPlaying &&
        <button
          onClick={() => (song || queue.next.length > 0) && play()}
          className={`${styles.controlButton} ${styles.playButton}`}
          disabled={!song && queue.next.length < 1}
        >
          <PlayIcon className={styles.controlIcon} />
        </button>}
      <button onClick={nextSong} className={`${styles.controlButton} ${styles.nextButton}`} disabled={queue.next.length < 1 || !isPlaying}>
        <NextIcon className={styles.controlIcon} />
      </button>
      <button onClick={stop} className={styles.controlButton}>
        <StopIcon className={styles.controlIcon} />
      </button>
      <button onClick={toggleQueue} className={`${styles.controlButton} ${styles.queueButton}`}>
        <QueueIcon className={styles.controlIcon} />
      </button>
    </div>
  )
}

export default Controls
