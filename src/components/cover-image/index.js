import React from 'react'
import { Link } from 'react-router-dom'

import PlayIcon from '../../icons/play.svg'
import styles from './styles.css'

const CoverImage = ({ link, image, imageClass = '', play }) => {
  return (
    <div className={styles.cover}>
      {(play || link) &&
        <div className={styles.overlay}>
          {link && <Link to={link} className={styles.link}></Link>}
          {play &&
            <button onClick={play} className={styles.playButton}>
              <PlayIcon className={styles.playIcon} height={100} width={100} />
            </button>}
        </div>}
      <img src={image} className={`${styles.image} ${imageClass}`} />
    </div>
  )
}

export default CoverImage
