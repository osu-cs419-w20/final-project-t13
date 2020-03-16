import React from 'react'
import { Link } from 'react-router-dom'

import RightChevronIcon from '../../icons/right-chevron.svg'
import LeftChevronIcon from '../../icons/left-chevron.svg'

import styles from './styles.css'

const Pagination = ({
  currentPage,
  totalPages,
  path,
  queryParam = 'page'
}) => {
  const hasPrevious = currentPage > 1
  const hasNext = currentPage < totalPages

  return (
    <div className={styles.pagination}>
      {hasPrevious &&
        <Link to={`${path}?${queryParam}=${currentPage - 1}`} className={styles.pageLink}>
          <LeftChevronIcon className={styles.pageIcon} height={25} width={25} viewBox="0 0 500 500" />
        </Link>}
      <p className={styles.pageNumber}>Page {currentPage} of {totalPages}</p>
      {hasNext &&
        <Link to={`${path}?${queryParam}=${currentPage + 1}`} className={styles.pageLink}>
          <RightChevronIcon className={styles.pageIcon} height={25} width={25} viewBox="0 0 500 500" />
        </Link>}
    </div>
  )
}

export default Pagination
