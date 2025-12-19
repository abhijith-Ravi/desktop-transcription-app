class PTTProcessor extends AudioWorkletProcessor {
  constructor() {
    super();
    this.buffer = [];
  }

  process(inputs) {
    const input = inputs[0]?.[0];
    if (!input) return true;

    let max = 0;

    for (let i = 0; i < input.length; i++) {
      const s = Math.max(-1, Math.min(1, input[i]));
      max = Math.max(max, Math.abs(s));
      this.buffer.push(s);
    }

    // 🔕 drop silence
    if (max < 0.003) {
      this.buffer = [];
      return true;
    }

    // 🎯 ~20ms of audio @16kHz = 320 samples
    if (this.buffer.length >= 160) {
      const pcm = new Int16Array(this.buffer.length);

      for (let i = 0; i < this.buffer.length; i++) {
        const s = this.buffer[i];
        pcm[i] = s < 0 ? s * 0x8000 : s * 0x7fff;
      }

      this.port.postMessage(
        { pcm: pcm.buffer, level: max },
        [pcm.buffer]
      );

      this.buffer = [];
    }

    return true;
  }
}

registerProcessor("ptt-processor", PTTProcessor);
