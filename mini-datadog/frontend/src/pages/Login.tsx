import React, { useState } from 'react';

export default function Login() {
  const [apiKey, setApiKey] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    // Placeholder for authenticating user or storing API key
    console.log('API Key submitted:', apiKey);
  };

  return (
    <div>
      <h1>Login / API Key</h1>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="Enter API Key"
          value={apiKey}
          onChange={(e) => setApiKey(e.target.value)}
        />
        <button type="submit">Submit</button>
      </form>
    </div>
  );
}