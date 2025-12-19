import { useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";

import "./style.css";



export default function TranscriptBox() {
  const [lines, setLines] = useState([]);
  const [error, setError] = useState(null);

  const lastRef = useRef(""); // track last transcript

  useEffect(() => {
    let unlistenTranscript;
    let unlistenError;

    // Listen for transcripts
    listen("transcript-final", (event) => {
      const text = event.payload;
      if (typeof text !== "string") return;

      // Ignore duplicates
      if (text === lastRef.current) return;
      lastRef.current = text;

      setLines(prev => [...prev, text]);
    }).then(fn => (unlistenTranscript = fn));

    // Listen for errors
    listen("app-error", (event) => {
      if (typeof event.payload === "string") {
        setError(event.payload);
      }
    }).then(fn => (unlistenError = fn));

    return () => {
      unlistenTranscript?.();
      unlistenError?.();
    };
  }, []);

  return (
    <div className="transcript-box">
      {/* Error UI */}
      {error && (
        <div className="alert alert-danger">
          {error}
        </div>
      )}


      {/*  Transcript UI */}
      {lines.length === 0 ? (
        <span className="transcript-placeholder">
          Transcription will appear here…
        </span>
      ) : (
        lines.map((line, i) => (
          <div key={i} className="transcript-line">
            {line}
          </div>
        ))
      )}
    </div>
  );
}
