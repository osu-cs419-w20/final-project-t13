import React from 'react'
import { Link } from 'react-router-dom'

import QueueItem from './queue-item'

import { displayTime } from '../../utils'

import styles from './styles.css'

const Queue = ({ close, isOpen, queue }) => {
  const className = `${styles.queue} ${isOpen ? styles.queueOpen : ''}`

  return (
    <div className={className}>
      <header className={styles.queueHeader}>
        <h4 className={styles.queueTitle}>Queue</h4>
        <button onClick={close} className={styles.queueCloseButton}>x</button>
      </header>

      {(!queue.next.length && !queue.prev.length && !queue.current) &&
        <p className={styles.queueEmptyMessage}>Your queue is empty.</p>}
      {(queue.next.length || queue.prev.length || queue.current) &&
        <ul className={styles.queueList}>
          {queue.prev.map((t, i) => <QueueItem key={`prev-${t.id}-${i}`} track={t} isPrev={true} />)}
          {queue.current && <QueueItem key={queue.current.id} track={queue.current} isCurrent={true} />}
          {queue.next.map((t, i) => <QueueItem key={`next-${t.id}-${i}`} track={t} />)}
        </ul>}
    </div>
  )
}

export default Queue
