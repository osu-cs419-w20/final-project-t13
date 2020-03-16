import React from 'react'

import CoverImage from '../../components/cover-image'

import Actions from './actions'
import Header from './header'
import Metadata from './metadata'
import TrackTable from './track-table'

import styles from './styles.css'

const AlbumPage = ({
  album,
  albumDuration,
  artist,
  tracks,
  play,
  queueAlbum,
  queueTrack,
  addToPlaylist,
  openPlaylistModal,
}) => {
  return (
    <div className={styles.wrapper}>
      {album &&
        <>
          <div className={styles.leftColumn}>
            <CoverImage image={album.image_url} imageClass={styles.albumImage} />

            <div className={styles.leftActionsData}>
              <Actions
                play={() => {
                  const [track, ...next] = tracks.map(t => t.id)
                  if (track) {
                    play(track, next ? next : [])
                  }
                }}
                queue={queueAlbum}
              />
              <Metadata duration={albumDuration} trackCount={tracks.length} />
            </div>
          </div>

          <div className={styles.rightColumn}>
            <Header title={album.title} artist={{ id: artist.id, name: artist.name }} />
            <TrackTable
              openPlaylistModal={openPlaylistModal}
              addToPlaylist={addToPlaylist}
              play={play}
              tracks={tracks}
              queueTrack={queueTrack}
            />
          </div>
        </>}
    </div>
  )
}

export default AlbumPage
