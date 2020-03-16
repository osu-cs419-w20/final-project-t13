import React from 'react'
import { useTable } from 'react-table'

import { displayTime } from '../../utils'

import Table from '../../components/table'
import { PlayButton, MoreButton } from './table-buttons'
import TrackMenu from './track-menu'

import styles from './styles.css'

const TrackTable = ({
  addToPlaylist,
  openPlaylistModal,
  play,
  queueTrack,
  tracks
}) => {
  const columns = React.useMemo(() => [
    { Header: '', accessor: 'play' },
    { Header: '#', accessor: 'position' },
    { Header: 'Song', accessor: 'title' },
    { Header: 'Duration', accessor: 'duration' },
    { Header: 'Bitrate', accessor: 'bitrate' },
    { Header: '', accessor: 'more' },
  ], [])
  
  const data = React.useMemo(() => (
    tracks.map(t => ({
      play: <PlayButton play={() => play(t.id, tracks.flatMap(it => it.position > t.position ? [it.id] : []))} />,
      position: t.position,
      title: t.title,
      duration: displayTime(t.duration),
      bitrate: t.bit_rate,
      more: (
        <TrackMenu
          id={t.id}
          queue={() => queueTrack(t.id)}
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
