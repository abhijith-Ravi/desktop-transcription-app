import { useRef, useState } from "react";
import { emit } from "@tauri-apps/api/event";
import "./style.css";


export default function PushToTalk() {
  const [recording, setRecording] = useState(false);

  const audioContextRef = useRef(null);
  const workletRef = useRef(null);
  const streamRef = useRef(null);

  async function startRecording() {
    if (recording) return;

    try {
      const stream = await navigator.mediaDevices.getUserMedia({
        audio: {
          channelCount: 1,
          echoCancellation: false,
          noiseSuppression: false,
          autoGainControl: false,
        },
      });

      // MUST MATCH BACKEND
      const audioContext = new AudioContext({ sampleRate: 16000 });

      await audioContext.audioWorklet.addModule("/ptt-processor.js");

      const source = audioContext.createMediaStreamSource(stream);
      const worklet = new AudioWorkletNode(audioContext, "ptt-processor");

      worklet.port.onmessage = (e) => {
        const { pcm, level } = e.data;

        // ignore silence
        if (level < 0.01) return;

        emit("audio-chunk", Array.from(new Uint8Array(pcm)));
      };

      source.connect(worklet);

      audioContextRef.current = audioContext;
      workletRef.current = worklet;
      streamRef.current = stream;

      emit("ptt-start"); // IMPORTANT
      setRecording(true);
      console.log("🎤 Recording started");
    } catch (err) {
      console.error("Failed to start recording", err);
    }
  }

  function stopRecording() {
    if (!recording) return;

    emit("ptt-stop"); // IMPORTANT

    workletRef.current?.disconnect();
    audioContextRef.current?.close();
    streamRef.current?.getTracks().forEach((t) => t.stop());

    workletRef.current = null;
    audioContextRef.current = null;
    streamRef.current = null;

    setRecording(false);
    console.log("Recording stopped");
  }

  function handleClick() {
    recording ? stopRecording() : startRecording();
  }

  return (
    <button
      className={`ptt-button ${recording ? "ptt-recording" : "ptt-idle"}`}
      onClick={handleClick}
    >
      {recording ? "Listening…" : "Push to Talk"}
    </button>
  );
}
