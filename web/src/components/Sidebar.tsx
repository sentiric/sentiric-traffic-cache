// File: web/src/components/Sidebar.tsx
import logo from '../assets/logo.svg';

type Page = 'dashboard' | 'network' | 'cache' | 'setup';

interface SidebarProps {
  activePage: Page;
  onNavigate: (page: Page) => void;
  pages: Page[];
}

export function Sidebar({ activePage, onNavigate, pages }: SidebarProps) {
  const pageLabels: Record<Page, string> = {
    dashboard: 'Dashboard',
    network: 'Network',
    cache: 'Cache',
    setup: 'Kurulum & Ayarlar',
  };

  return (
    <nav style={{ width: '220px', background: '#002b49', color: 'white', padding: '20px', display: 'flex', flexDirection: 'column' }}>
      <div style={{ marginBottom: '40px', display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '10px' }}>
        <img src={logo} alt="VeloCache Logo" style={{ width: '32px', height: '32px' }} />
        <h2 style={{ margin: 0, fontSize: '1.5rem' }}>VeloCache</h2>
      </div>
      <ul style={{ listStyleType: 'none', padding: 0, margin: 0 }}>
        {pages.map(page => (
          <li 
            key={page} 
            onClick={() => onNavigate(page)}
            style={{ 
              padding: '12px 15px', 
              cursor: 'pointer', 
              background: activePage === page ? '#005a9c' : 'transparent',
              borderRadius: '8px',
              marginBottom: '5px',
              transition: 'background 0.2s ease-in-out',
            }}
          >
            {pageLabels[page]}
          </li>
        ))}
      </ul>
      <div style={{ marginTop: 'auto', fontSize: '0.8em', textAlign: 'center', opacity: 0.6 }}>
        <p>Status: <span style={{ color: '#4ade80', fontWeight: 'bold' }}>Active</span></p>
        <p>v1.0.0</p>
      </div>
    </nav>
  );
}