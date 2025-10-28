// File: web/src/main.tsx
import { render } from 'preact';
import { App } from './app'; // <- DEĞİŞİKLİK BURADA: './App' -> './app'
import './index.css';
import './pages/Spinner.css';

// AppContext'e artık ihtiyaç yok, store kendini yönetiyor.
render(<App />, document.getElementById('app')!);