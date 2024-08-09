// src/UnderConstruction.tsx
import { Result } from 'antd';
import { SmileOutlined } from '@ant-design/icons';
import i18next from 'i18next';

export default function TodoPage() {
    const t = i18next.t;
    return (
        <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh' }}>
            <Result
                icon={<SmileOutlined />}
                title={t('message:Message-Todo-Page-Tittle')}
                subTitle={t('message:Message-Todo-Page-SubTittle')}
            />
        </div>
    );
};

