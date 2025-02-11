import React from 'react';
import LiveLogTailing from '../components/LiveLogTailing';
import MetricsDashboard from '../components/MetricsDashboard';
import TracesView from '../components/TracesView';
import AlertsPanel from '../components/AlertsPanel';

export default function Home() {
  return (
    <div>
      <h1>Mini Datadog Dashboard</h1>
      <MetricsDashboard />
      <TracesView />
      <AlertsPanel />
      <LiveLogTailing />
    </div>
  );
}