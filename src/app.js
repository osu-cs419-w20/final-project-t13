import React from 'react'
import { Route, Switch } from 'react-router-dom'

import AlbumRoute from './routes/album'
import AlbumsRoute from './routes/albums'
import ArtistRoute from './routes/artist'
import ArtistsRoute from './routes/artists'
import HomeRoute from './routes/home'
import PlaylistRoute from './routes/playlist'

import MusicPlayer from './components/music-player'
import Sidebar from './components/sidebar'

import NotFoundPage from './pages/not-found'

const App = () => {
  return (
    <div>
      <Sidebar />
      <MusicPlayer />

      <Switch>
        <Route exact path="/albums/:albumId" component={AlbumRoute} />
        <Route exact path="/albums" component={AlbumsRoute} />
        <Route exact path="/artists/:artistId" component={ArtistRoute} />
        <Route exact path="/artists" component={ArtistsRoute} />
        <Route exact path="/playlists/:playlistId" component={PlaylistRoute} />
        <Route exact path="/" component={HomeRoute} />
        <Route component={NotFoundPage} />
      </Switch>
    </div>
  )
}

export default App
