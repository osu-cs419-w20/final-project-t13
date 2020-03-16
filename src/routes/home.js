import React from 'react'

import Page from '../components/page'

const HomeRoute = (props) => {
  const styles = {
    fontSize: '1.1em',
    fontWeight: 500,
    padding: '35px'
  }
  return (
    <Page {...props} pageTitle="Home">
      <h2 style={styles}>Select a page from the sidebar to start listening!</h2>
    </Page>
  )
}

export default HomeRoute
