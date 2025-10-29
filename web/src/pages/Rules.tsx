import { useState, useEffect } from 'preact/hooks';
import * as api from '../api';
import type { Rule, RuleCondition } from '../api';

const formatCondition = (condition: RuleCondition) => {
    if ('domain' in condition) {
        return `Domain: ${condition.domain}`;
    }
    if ('urlPattern' in condition) {
        return `URL Deseni: ${condition.urlPattern}`;
    }
    return 'Bilinmeyen';
};

const formatAction = (action: api.Action) => {
    switch (action) {
        case 'Allow': return { text: 'İZİN VER', color: '#28a745' };
        case 'Block': return { text: 'ENGELLE', color: '#dc3545' };
        case 'BypassCache': return { text: 'ÖNBELLEĞİ ATLA', color: '#ffc107' };
        default: return { text: action, color: '#6c757d' };
    }
};

export function Rules() {
    const [rules, setRules] = useState<Rule[]>([]);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        api.fetchRules()
           .then(setRules)
           .catch(err => setError(err.toString()));
    }, []);

    return (
        <div>
            <h1>Kural Motoru</h1>
            <div class="section">
                <p style={{ marginTop: 0, color: '#6c757d' }}>
                    Aktif kurallar aşağıda listelenmiştir. Kurallar, <code>rules.toml</code> dosyasından okunur ve yukarıdan aşağıya doğru sırayla işlenir. Bir istekle eşleşen ilk kural uygulanır.
                </p>

                {error && <p style={{color: 'red'}}>Kurallar yüklenirken hata oluştu: {error}</p>}

                <table>
                    <thead>
                        <tr>
                            <th>Kural Adı</th>
                            <th>Koşul</th>
                            <th>Eylem</th>
                        </tr>
                    </thead>
                    <tbody>
                        {rules.length === 0 && !error ? (
                            <tr>
                                <td colSpan={3} style={{ textAlign: 'center', padding: '40px' }}>Aktif kural bulunamadı.</td>
                            </tr>
                        ) : (
                            rules.map(rule => {
                                const actionStyle = formatAction(rule.action);
                                return (
                                    <tr key={rule.name}>
                                        <td>{rule.name}</td>
                                        <td>{formatCondition(rule.condition)}</td>
                                        <td>
                                            <span style={{ fontWeight: 'bold', color: actionStyle.color }}>
                                                {actionStyle.text}
                                            </span>
                                        </td>
                                    </tr>
                                );
                            })
                        )}
                    </tbody>
                </table>
            </div>
        </div>
    );
}