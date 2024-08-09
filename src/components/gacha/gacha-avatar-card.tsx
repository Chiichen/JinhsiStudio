import { useState } from 'react';
import './gacha-avatar-card.css';
import { QuestionOutlined } from "@ant-design/icons";
import { matchResource } from './gacha-resource';



interface gachaAvatarCardProps {
    name: string;
    resourceId: number;
    number: number;
}

export default function GachaAvatarCard(props: gachaAvatarCardProps) {
    const { name, resourceId, number } = props;
    const imagePath = matchResource(name, resourceId);
    const [imageError, setImageError] = useState(false);
    const handleError = () => {
        console.error('Cannot find resource')
        setImageError(true);
    };
    console.info(imagePath, name, resourceId);
    return (
        <div className="gacha-avatar-card">
            {imageError || !imagePath ? (
                <QuestionOutlined className="placeholder-icon" />
            ) : (
                <img src={imagePath} alt="small" className="gacha-avatar" onError={handleError} />
            )}
            <div className="number">{number}</div>
        </div>
    );
}


