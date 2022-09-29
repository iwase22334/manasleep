import { useState } from 'react'
import './App.css'
import MusicPlayerSlider from './musicplayer/musicPlayer';
import { ContextProvider } from './musicplayer/PlayerContext';
import { PlayerDrawer } from './musicplayer/SettingMenu';

function App() {

  return (
    <ContextProvider>
      <MusicPlayerSlider />
      <PlayerDrawer />
    </ContextProvider>
  )
}

export default App
