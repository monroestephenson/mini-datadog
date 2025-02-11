import React, { useState } from 'react';
import { 
  List, ListItem, ListItemText, ListItemIcon, 
  IconButton, Button, Dialog, DialogTitle, 
  DialogContent, TextField, DialogActions 
} from '@mui/material';
import ErrorIcon from '@mui/icons-material/Error';
import DeleteIcon from '@mui/icons-material/Delete';
import AddIcon from '@mui/icons-material/Add';

interface Alert {
  id: number;
  name: string;
  condition: string;
  threshold: number;
  status: 'active' | 'resolved';
}

export default function AlertsPanel() {
  const [alerts, setAlerts] = useState<Alert[]>([
    { id: 1, name: 'High CPU Usage', condition: 'CPU > 90%', threshold: 90, status: 'active' },
    { id: 2, name: 'Memory Warning', condition: 'Memory > 80%', threshold: 80, status: 'resolved' },
  ]);
  const [open, setOpen] = useState(false);
  const [newAlert, setNewAlert] = useState({ name: '', condition: '', threshold: 0 });

  const handleDelete = (id: number) => {
    setAlerts(alerts.filter(alert => alert.id !== id));
  };

  const handleAdd = () => {
    setAlerts([...alerts, { ...newAlert, id: Date.now(), status: 'active' }]);
    setOpen(false);
    setNewAlert({ name: '', condition: '', threshold: 0 });
  };

  return (
    <div className="dashboard-card">
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <h2 className="card-title">Alerts</h2>
        <Button startIcon={<AddIcon />} variant="contained" onClick={() => setOpen(true)}>
          New Alert
        </Button>
      </div>
      
      <List>
        {alerts.map((alert) => (
          <ListItem key={alert.id} 
                    sx={{ bgcolor: alert.status === 'active' ? 'rgba(255,0,0,0.1)' : 'inherit' }}>
            <ListItemIcon>
              <ErrorIcon color={alert.status === 'active' ? 'error' : 'disabled'} />
            </ListItemIcon>
            <ListItemText 
              primary={alert.name}
              secondary={alert.condition}
            />
            <IconButton onClick={() => handleDelete(alert.id)}>
              <DeleteIcon />
            </IconButton>
          </ListItem>
        ))}
      </List>

      <Dialog open={open} onClose={() => setOpen(false)}>
        <DialogTitle>Create New Alert</DialogTitle>
        <DialogContent>
          <TextField
            autoFocus
            margin="dense"
            label="Alert Name"
            fullWidth
            value={newAlert.name}
            onChange={(e) => setNewAlert({...newAlert, name: e.target.value})}
          />
          <TextField
            margin="dense"
            label="Condition"
            fullWidth
            value={newAlert.condition}
            onChange={(e) => setNewAlert({...newAlert, condition: e.target.value})}
          />
          <TextField
            margin="dense"
            label="Threshold"
            type="number"
            fullWidth
            value={newAlert.threshold}
            onChange={(e) => setNewAlert({...newAlert, threshold: Number(e.target.value)})}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpen(false)}>Cancel</Button>
          <Button onClick={handleAdd} variant="contained">Create</Button>
        </DialogActions>
      </Dialog>
    </div>
  );
}