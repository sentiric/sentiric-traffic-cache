import logo from '../assets/logo.svg';

type Page = 'dashboard';

interface SidebarProps {
  activePage: Page;
  onNavigate: (page: Page) => void;
  pages: Page[];
}

export function Sidebar({ activePage, onNavigate, pages }: SidebarProps) {
  const pageLabels: Record<Page, string> = {
    dashboard: 'Gösterge Paneli',
  };

  return (
    <nav style={{ width: '220px', background: '#002b49', color: 'white', padding: '20px', display: 'flex', flexDirection: 'column' }}>
      <div style={{ marginBottom: '40px', display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '10px' }}>
        <img src={logo} alt="Logo" style={{ width: '32px', height: '32px' }} />
        <h2 style={{ margin: 0, fontSize: '1.5rem' }}>Cache</h2>
      </div>
      <ul style={{ listStyleType: 'none', padding: 0, margin: 0 }}>
        {pages.map(page => (
          <li key={page} onClick={() => onNavigate(page)}
            style={{ padding: '12px 15px', cursor: 'pointer', background: activePage === page ? '#005a9c' : 'transparent', borderRadius: '8px' }}>
            {pageLabels[page]}
          </li>
        ))}
      </ul>
    </nav>
  );
}