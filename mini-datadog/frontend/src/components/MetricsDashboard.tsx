import React, { useState, useEffect } from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend } from 'recharts';
import { Card, CardContent, Typography, Grid } from '@mui/material';
import MemoryIcon from '@mui/icons-material/Memory';
import SpeedIcon from '@mui/icons-material/Speed';
import StorageIcon from '@mui/icons-material/Storage';
import axios from 'axios';

interface Metric {
    time: string;
    cpu: number;
    memory: number;
    latency: number;
    request_count: number;
}

export default function MetricsDashboard() {
    const [metrics, setMetrics] = useState<Metric[]>([]);

    useEffect(() => {
        // Initial fetch
        fetchMetrics();

        // Poll for updates every second
        const interval = setInterval(fetchMetrics, 1000);
        return () => clearInterval(interval);
    }, []);

    const fetchMetrics = async () => {
        try {
            const response = await axios.get<Metric[]>('http://localhost:8080/api/metrics');
            setMetrics(response.data);
        } catch (error) {
            console.error('Failed to fetch metrics:', error);
        }
    };

    return (
        <div className="dashboard-card">
            <h2 className="card-title">System Metrics</h2>
            <Grid container spacing={2}>
                <Grid item xs={4}>
                    <Card>
                        <CardContent>
                            <Typography color="textSecondary" gutterBottom>
                                <MemoryIcon /> CPU Usage
                            </Typography>
                            <Typography variant="h4">
                                {metrics[metrics.length - 1]?.cpu.toFixed(1)}%
                            </Typography>
                        </CardContent>
                    </Card>
                </Grid>
                <Grid item xs={4}>
                    <Card>
                        <CardContent>
                            <Typography color="textSecondary" gutterBottom>
                                <StorageIcon /> Memory
                            </Typography>
                            <Typography variant="h4">
                                {metrics[metrics.length - 1]?.memory.toFixed(1)}GB
                            </Typography>
                        </CardContent>
                    </Card>
                </Grid>
                <Grid item xs={4}>
                    <Card>
                        <CardContent>
                            <Typography color="textSecondary" gutterBottom>
                                <SpeedIcon /> Latency
                            </Typography>
                            <Typography variant="h4">
                                {metrics[metrics.length - 1]?.latency.toFixed(0)}ms
                            </Typography>
                        </CardContent>
                    </Card>
                </Grid>
                <Grid item xs={12}>
                    <LineChart width={600} height={300} data={metrics}>
                        <CartesianGrid strokeDasharray="3 3" />
                        <XAxis dataKey="time" />
                        <YAxis />
                        <Tooltip />
                        <Legend />
                        <Line type="monotone" dataKey="cpu" stroke="#8884d8" name="CPU %" />
                        <Line type="monotone" dataKey="memory" stroke="#82ca9d" name="Memory (GB)" />
                        <Line type="monotone" dataKey="latency" stroke="#ffc658" name="Latency (ms)" />
                    </LineChart>
                </Grid>
            </Grid>
        </div>
    );
}