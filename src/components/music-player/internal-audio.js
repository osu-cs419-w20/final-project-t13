import React from 'react'

const InternalAudio = ({
  currentSong,
  isPlaying,
  pause,
  play,
  finish,
  setCurrentTime,
  setDuration,
}) => {
  const audioRef = React.createRef(null)

  React.useEffect(() => {
    const ref = audioRef.current
    if (isPlaying) {
      ref.play()
    } else {
      ref.pause()
    }
  }, [isPlaying])

  React.useEffect(() => {
    if (currentSong) {
      const ref = audioRef.current
      ref.load()
      // If the player is already playing when the song changes,
      // the effect above won't trigger
      if (isPlaying) {
        ref.play()
      } else {
        play()
      }
    }
  }, [currentSong])

  return (
    <audio
      ref={audioRef}
      onTimeUpdate={() => setCurrentTime(audioRef.current.currentTime)}
      onDurationChange={() => setDuration(audioRef.current.duration)}
      onEnded={finish}
    >
      <source src={`/api/play/${currentSong.id}`} type="audio/flac" />
    </audio>
  )
}

export default InternalAudio
