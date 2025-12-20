# Push-to-Talk Desktop Transcription App (Tauri + Deepgram)

A desktop push-to-talk transcription application built with Tauri that captures microphone audio, streams it to Deepgram in real time, and displays accurate, low-latency speech-to-text transcription.

This project prioritizes real-time streaming correctness, clean architecture, and maintainability over UI polish.

---

## Demo video

https://drive.google.com/file/d/1svEMPbIviM36lMLzAKqXK8wn6zfL-flp/view?usp=drive_link

## Introduction

This app enables push-to-talk voice input with real-time transcription using Deepgram’s WebSocket API. Audio is captured from the microphone, streamed efficiently as PCM16, and transcriptions are returned and displayed with minimal latency.

It is designed as a working, production-ready prototype demonstrating proper handling of real-time audio streaming and transcription pipelines.

---

## Features

- Push-to-talk voice input
- Real-time transcription via Deepgram
- Final-only transcript handling (no duplicate text)
- Cleanly appended transcription segments
- Clear start/stop recording controls with visual feedback
- Basic error handling
  - Microphone access failures
  - WebSocket connection issues

---

## Tech Stack

| Layer | Technology |
|------|-----------|
| Desktop Framework | Tauri (Rust backend) |
| Frontend UI | React |
| Audio Capture | Web Audio API (AudioWorkletProcessor) |
| Transcription | Deepgram WebSocket API |
| Audio Format | PCM16 |

---

## Architecture Overview

The application follows a clear separation of concerns for readability and long-term maintainability.

### Architecture Flow

React UI  
↓  
AudioWorkletProcessor  
↓  
Tauri (Rust)  
↓  
Deepgram WebSocket API  

### UI Layer (React)
- Handles push-to-talk controls
- Displays live and final transcripts

### Audio Capture Layer
- Uses AudioWorkletProcessor
- Captures microphone input
- Converts audio to PCM16 format

### Transcription Service (Rust / Tauri)
- Streams audio to Deepgram via WebSocket
- Receives transcription results
- Emits final transcripts only back to the UI

---

## Setup & Run

### 1. Install dependencies

```bash
npm install
```
### 2. Set Deepgram API Key
### macOS / Linux

```bash
export DEEPGRAM_API_KEY=your_key_here
```
### Windows (PowerShell)

```bash
setx DEEPGRAM_API_KEY "your_key_here"
```
### 3. Run the Application
```bash
npm run tauri dev
```


---

## Usage

- Launch the application.  
- Click the Push-to-Talk button to start recording.  
- Speak into the microphone.  
- Click the button again to stop recording. 
- View real-time transcription output in the UI.

---

## Environment Variables

| Variable | Description |
|-----------|--------------|
| `DEEPGRAM_API_KEY` | Deepgram API key used for transcription |

---

## Status

- Core push-to-talk workflow implemented  
- End-to-end real-time transcription functional  
- Clean, maintainable architecture  
- Ready for evaluation and extension

---

## Troubleshooting

### Microphone Not Working
> Ensure microphone permissions are granted.  
> Verify OS-level input device settings.

### No Transcription Output
> Confirm the `DEEPGRAM_API_KEY` is correctly set.  
> Ensure outbound WebSocket connections are allowed.

### High Latency
> Check network stability.  
> Avoid heavy background CPU usage.

---

