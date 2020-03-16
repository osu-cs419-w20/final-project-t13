import React from 'react'

import LeftArrowIcon from '../../icons/left-arrow.svg'
import styles from './styles.css'

const Page = ({
  children,
  history,
  pageTitle,
  wrapperPageTitle
}) => {
  return (
    <div className={styles.pageWrapper}>
      <header className={styles.header}>
        <button onClick={() => history.goBack()} className={styles.button}>
          <LeftArrowIcon className={styles.icon} height={25} width={40} viewBox="0 1 30 30" />
        </button>

        <h2 className={styles.pageTitle}>{wrapperPageTitle || pageTitle}</h2>
      </header>

      <main className={styles.main}>
        {children}
      </main>
    </div>
  )
}

export default Page
