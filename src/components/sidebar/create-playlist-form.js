import React from 'react'

import styles from './styles.css'

const CreatePlaylistForm = ({ create }) => {
  const [name, setName] = React.useState('')
  const [badInput, setBadInput] = React.useState(false)

  const submit = e => {
    e.preventDefault()

    if (name == '') {
      setBadInput(true)
      return
    }

    if (badInput) {
      setBadInput(false)
    }

    create(name)
  }

  return (
    <form onSubmit={submit} className={styles.createPlaylistForm}>
      <input type="text" placeholder="Playlist Name" value={name} onChange={e => setName(e.target.value)} />
      {badInput && <p className={styles.error}>Playlist name cannot be empty.</p>}
    </form>
  )
}

export default CreatePlaylistForm
