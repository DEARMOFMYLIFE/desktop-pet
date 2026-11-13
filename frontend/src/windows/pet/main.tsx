import React from 'react';
import ReactDOM from 'react-dom/client';
import '../../styles/global.css';
import { PetWindow } from './PetWindow';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <PetWindow />
  </React.StrictMode>,
);
