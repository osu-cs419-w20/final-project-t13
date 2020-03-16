import React from 'react'
import { Link } from 'react-router-dom'
import { useTable } from 'react-table'

import { displayTime } from '../../utils'

import Table from '../../components/table'
import CoverImage from '../../components/cover-image'
import { PlayButton, MoreButton } from './table-buttons'
import TrackMenu from './track-menu'

import styles from './styles.css'

const TrackTable = ({
  play,
  queue,
  tracks,
  addToPlaylist,
  openPlaylistModal,
}) => {
  const columns = React.useMemo(() => [
    { Header: '', accessor: 'play' },
    { Header: '', accessor: 'albumCover' },
    { Header: 'Song', accessor: 'track' },
    { Header: 'Duration', accessor: 'duration' },
    { Header: 'Artist', accessor: 'artist' },
    { Header: '', accessor: 'more' },
  ], [])
  
  const data = React.useMemo(() => (
    tracks.map(t => ({
      play: <PlayButton play={() => play(t, tracks.flatMap(it => it.position > t.position ? [it.id] : []))} />,
      albumCover: <CoverImage image={t.albumImage} imageClass={styles.albumImage} />,
      track: (
        <div className={styles.tableTrack}>
          <p className={styles.tableTrackTitle}>{t.title}</p>
          <Link to={`/albums/${t.albumId}`} className={styles.tableTrackAlbumLink}>{t.albumTitle}</Link>
        </div>
      ),
      duration: displayTime(t.duration),
      artist: <Link to={`/artists/${t.artistId}`} className={styles.tableTrackArtistLink}>{t.artistName}</Link>,
      more: (
        <TrackMenu
          id={t.id}
          queue={() => queue(t.id)}
          openPlaylistModal={openPlaylistModal}
          addToPlaylist={() => addToPlaylist(t.id)}
        >
          <MoreButton open={() => {}} />
        </TrackMenu>
      )
    }))
  ), [tracks])

  return <Table columns={columns} data={data} />
}

export default TrackTable
