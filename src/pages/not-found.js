import React from 'react'

import Page from '../components/page'

const NotFoundPage = (props) => {
  const styles = {
    fontSize: '2.5vw',
    fontWeight: 700,
    padding: '50px'
  }

  return (
    <Page {...props} pageTitle="404: Page Not Found">
      <h2 style={styles}>404: Page Not Found</h2>
    </Page>
  )
}

export default NotFoundPage
