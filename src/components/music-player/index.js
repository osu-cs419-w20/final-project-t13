import React from 'react'
import { useDispatch, useSelector } from 'react-redux';

import {
  getCurrentTrack,
  getPlayerState,
  getQueue,
} from '../../reducers/music-player';
import {
  nextTrack,
  pauseMusic,
  playMusic,
  prevTrack,
  setCurrentTime,
  setDuration,
  stopMusic,
} from '../../actions/music-player'

import Controls from './controls'
import CurrentSong from './current-song'
import InternalAudio from './internal-audio'
import ProgressBar from './progress-bar'
import Queue from './queue'
import Timer from './timer'

import styles from './styles.css'

const MusicPlayer = () => {
  const dispatch = useDispatch()
  const playerState = useSelector(getPlayerState)
  const queue = useSelector(getQueue)
  const { album, track } = useSelector(getCurrentTrack) || {}

  const [queueOpen, setQueueOpen] = React.useState(false)

  const finish = () => {
    if (queue.next.length > 1) {
      dispatch(nextTrack())
    } else {
      dispatch(pauseMusic())
    }
  }

  const playPrevious = () => {
    if (prevTrack) {
      dispatch(prevTrack())
    }
  }

  const playNext = () => {
    if (nextTrack) {
      dispatch(nextTrack())
    }
  }

  return (
    <div className={styles.musicPlayer}>
      {track && <InternalAudio
        currentSong={track}
        isPlaying={playerState.playing}
        pause={() => dispatch(pauseMusic())}
        play={() => dispatch(playMusic())}
        finish={finish}
        setCurrentTime={t => dispatch(setCurrentTime(t))}
        setDuration={d => dispatch(setDuration(d))}
      />}
      <ProgressBar currentTime={playerState.currentTime} duration={playerState.duration} />

      <div className={styles.playerBar}>
        <CurrentSong song={track} album={album} />
        <Controls
          song={track}
          stop={() => dispatch(stopMusic())}
          prevSong={playPrevious}
          nextSong={playNext}
          isPlaying={playerState.playing}
          pause={() => dispatch(pauseMusic())}
          play={() => dispatch(playMusic())}
          toggleQueue={() => setQueueOpen(!queueOpen)}
          queue={queue}
        />
        <Timer currentTime={playerState.currentTime} duration={playerState.duration} />
        <Queue queue={queue} isOpen={queueOpen} close={() => setQueueOpen(false)} />
      </div>
    </div>
  )
}

export default MusicPlayer
