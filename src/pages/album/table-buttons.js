import React from 'react'

import PlayIcon from '../../icons/play.svg'
import MoreMenuIcon from '../../icons/three-dots.svg'
import styles from './styles.css'

const PlayButton = ({ play }) => {
  return (
    <button onClick={play} className={styles.tableButton}>
      <PlayIcon className={styles.tableIcon} height={25} width={25} />
    </button>
  )
}

const MoreButton = ({ open }) => {
  return (
    <button onClick={open} className={styles.tableButton}>
      <MoreMenuIcon className={styles.tableIcon} height={20} width={30} viewBox="-125 0 750 500" />
    </button>
  )
}

export { PlayButton, MoreButton }
