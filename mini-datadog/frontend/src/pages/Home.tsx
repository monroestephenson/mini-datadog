import React from 'react';
import LiveLogTailing from '../components/LiveLogTailing';
import MetricsDashboard from '../components/MetricsDashboard';
import TracesView from '../components/TracesView';
import AlertsPanel from '../components/AlertsPanel';
import '../styles/global.css';

export default function Home() {
  return (
    <div className="dashboard-container">
      <div className="dashboard-header">
        <h1>Mini Datadog Dashboard</h1>
      </div>
      <div className="dashboard-grid">
        <MetricsDashboard />
        <AlertsPanel />
        <TracesView />
        <LiveLogTailing />
      </div>
    </div>
  );
}