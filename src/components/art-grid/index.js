import React from 'react'

import styles from './styles.css'

const ArtGrid = ({ children, className, perRow = 4 }) => {
  const liClass = `${styles.listItem} ${styles[`listItem--${perRow}`]}`
  return (
    <ul className={`${styles.list} ${className || ''}`}>
      {React.Children.map(children, child => {
        const className = `${liClass} ${child.props.className || ''}`
        return React.cloneElement(child, { className })
      })}
    </ul>
  )
}

export default ArtGrid
