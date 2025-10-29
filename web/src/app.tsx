import { ComponentChild } from 'preact';
import { useState } from 'preact/hooks';
import { Sidebar } from './components/Sidebar';
import { Dashboard } from './pages/Dashboard';
import { Settings } from './pages/Settings';
import { ConnectDevice } from './pages/ConnectDevice';
import { NetworkFlow } from './pages/NetworkFlow';
import { Rules } from './pages/Rules'; // YENİ

type Page = 'dashboard' | 'network_flow' | 'rules' | 'connect_device' | 'settings'; // YENİ
const pageComponents: Record<Page, () => ComponentChild> = {
  dashboard: Dashboard,
  network_flow: NetworkFlow,
  rules: Rules, // YENİ
  connect_device: ConnectDevice,
  settings: Settings,
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