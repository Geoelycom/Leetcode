import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App.jsx'
import fruits from './data.js';

createRoot(document.getElementById('root')).render(
  <StrictMode>
    {/* Pass fruits as props to App component */}
    <App fruits={fruits}/>
  </StrictMode>,
)
