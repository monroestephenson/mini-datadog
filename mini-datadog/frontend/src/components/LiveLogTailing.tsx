import React, { useEffect, useState } from 'react';

export default function LiveLogTailing() {
  const [logs, setLogs] = useState<string[]>([]);

  useEffect(() => {
    // Placeholder WebSocket connection
    const ws = new WebSocket('ws://localhost:8080/logs/live');

    ws.onmessage = (evt) => {
      setLogs((prevLogs) => [...prevLogs, evt.data]);
    };

    // Add some mock logs for demonstration
    const mockLogs = [
      '[INFO] Server started successfully',
      '[DEBUG] Connected to database',
      '[INFO] Processing request #1234',
      '[WARN] High memory usage detected',
      '[ERROR] Failed to connect to cache'
    ];

    setLogs(mockLogs);

    return () => {
      ws.close();
    };
  }, []);

  return (
    <div className="dashboard-card">
      <h2 className="card-title">Live Log Tailing</h2>
      <div className="logs-container">
        {logs.map((log, idx) => (
          <div key={idx} className="log-entry">{log}</div>
        ))}
      </div>
    </div>
  );
}