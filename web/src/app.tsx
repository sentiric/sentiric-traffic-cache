// File: web/src/app.tsx
import { ComponentChild } from 'preact';
import { useState } from 'preact/hooks';
import { Sidebar } from './components/Sidebar';
import { Dashboard } from './pages/Dashboard';
import { Network } from './pages/Network';
import { Cache } from './pages/Cache';
import { Setup } from './pages/Setup';

type PageComponent = () => ComponentChild;
type Page = 'dashboard' | 'network' | 'cache' | 'setup';

const pageComponents: Record<Page, PageComponent> = {
  dashboard: Dashboard,
  network: Network,
  cache: Cache,
  setup: Setup,
};

export function App() {
  const [activePage, setActivePage] = useState<Page>('dashboard');
  const ActivePageComponent = pageComponents[activePage];

  return (
    <div style={{ display: 'flex' }}>
      <Sidebar 
        activePage={activePage} 
        onNavigate={setActivePage}
        pages={Object.keys(pageComponents) as Page[]}
      />
      <main style={{ flex: 1, padding: '40px', overflowY: 'auto', background: '#f7f8fc' }}>
        <ActivePageComponent />
      </main>
    </div>
  );
}