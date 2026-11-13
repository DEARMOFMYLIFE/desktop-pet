import React from 'react';
import ReactDOM from 'react-dom/client';
import '../../styles/global.css';
import { BubbleWindow } from './BubbleWindow';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BubbleWindow />
  </React.StrictMode>,
);
