import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css'; // You can create a basic index.css if needed
import App from './App';
import { BrowserRouter } from 'react-router-dom';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <React.StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </React.StrictMode>
);

// reportWebVitals(); // REMOVED 