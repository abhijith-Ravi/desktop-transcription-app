import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import PushToTalk from './components/PushToTalk'
import TranscriptBox from './components/TranscriptBox'

function App() {


  return (
    <div style={{ padding: "24px" }}>
      <TranscriptBox />
      <PushToTalk />
    </div>
  )
}

export default App
