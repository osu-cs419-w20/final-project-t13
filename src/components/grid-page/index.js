import React from 'react'

import Page from '../page'

import styles from './styles.css'

const GridPage = (props) => {
  return (
    <Page {...props}>
      <div className={styles.wrapper}>
        <header className={styles.pageHeader}>
          <h1 className={styles.pageTitle}>{props.pageTitle}</h1>
        </header>

        {props.children}
      </div>
    </Page>
  )
}

export default GridPage
