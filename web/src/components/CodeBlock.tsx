// File: web/src/components/CodeBlock.tsx
import { useState } from 'preact/hooks';

export function CodeBlock({ code }: { code: string }) {
  const [buttonText, setButtonText] = useState('Kopyala');

  const handleCopy = () => {
    navigator.clipboard.writeText(code.trim());
    setButtonText('Kopyalandı!');
    setTimeout(() => setButtonText('Kopyala'), 2000);
  };

  return (
    <div style={{ position: 'relative', background: '#2d333b', borderRadius: '8px', marginBottom: '20px' }}>
      <pre style={{
        color: '#cdd9e5',
        padding: '20px',
        overflowX: 'auto',
        whiteSpace: 'pre-wrap',
        wordWrap: 'break-word',
        fontFamily: 'monospace',
        fontSize: '0.9rem',
      }}>
        <code>{code.trim()}</code>
      </pre>
      <button 
        onClick={handleCopy}
        style={{
          position: 'absolute',
          top: '10px',
          right: '10px',
          background: '#444c56',
          color: '#cdd9e5',
          border: '1px solid #555',
          borderRadius: '6px',
          padding: '5px 10px',
          cursor: 'pointer',
          transition: 'background 0.2s',
        }}
      >
        {buttonText}
      </button>
    </div>
  );
}