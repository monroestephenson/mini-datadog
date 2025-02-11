import React, { useEffect, useState } from 'react';

export default function LiveLogTailing() {
  const [logs, setLogs] = useState<string[]>([]);

  useEffect(() => {
    // Placeholder WebSocket connection
    const ws = new WebSocket('ws://localhost:8080/logs/live');

    ws.onmessage = (evt) => {
      setLogs((prevLogs) => [...prevLogs, evt.data]);
    };

    return () => {
      ws.close();
    };
  }, []);

  return (
    <div>
      <h2>Live Log Tailing</h2>
      <div style={{ maxHeight: '200px', overflowY: 'scroll' }}>
        {logs.map((log, idx) => (
          <div key={idx}>{log}</div>
        ))}
      </div>
    </div>
  );
}