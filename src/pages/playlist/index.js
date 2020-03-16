import React from 'react'

import { displayTime } from '../../utils'

import Actions from './actions'
import TrackTable from './track-table'

import styles from './styles.css'

const PlaylistPage = ({
  play,
  playlist,
  queue,
  openPlaylistModal,
  addToPlaylist,
}) => {
  const tracks = (playlist && playlist.tracks) ? playlist.tracks : []
  const duration = tracks.reduce((acc, t) => acc + t.duration, 0)

  return (
    <div className={styles.wrapper}>
      {playlist &&
        <>
          <section className={styles.leftColumn}>
            <div className={styles.playlistInfo}>
              <span className={styles.playlistLabel}>Playlist</span>
              <h1 className={styles.playlistName}>{playlist.name}</h1>

              <Actions
                play={() => {
                  const [first, ...remaining] = tracks
                  if (first) {
                    play(first, remaining.map(t => t.id))
                  }
                }}
                queue={() => queue(tracks.map(t => t.id))}
              />

              <div className={styles.metadata}>
                <span className={styles.metadataItem}>{tracks.length} songs</span>
                <span className={styles.metadataItem}>{displayTime(duration)}</span>
              </div>
            </div>
          </section>

          <section className={styles.rightColumn}>
            <TrackTable
              play={play}
              tracks={tracks}
              queue={queue}
              openPlaylistModal={openPlaylistModal}
              addToPlaylist={addToPlaylist}
            />
          </section>
        </>}
    </div>
  )
}

export default PlaylistPage
