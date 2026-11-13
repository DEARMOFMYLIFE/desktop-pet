import React from 'react';
import ReactDOM from 'react-dom/client';
import '../../styles/global.css';
import { OnboardingWindow } from './OnboardingWindow';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <OnboardingWindow />
  </React.StrictMode>,
);
