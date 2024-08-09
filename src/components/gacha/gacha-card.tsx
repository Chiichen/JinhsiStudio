import { Convene } from '@/models/gacha/convene';
import { GachaItem } from '@/models/gacha/gacha-item';
import { GachaLog } from '@/models/gacha/gacha-log';
import { Card, Tabs, Statistic } from 'antd';
import GachaAvatarCard from './gacha-avatar-card';
import './gacha-card.css';

const { TabPane } = Tabs;

interface GachaCardProps {
    data: GachaLog,
}

const calculateDistance = (items: GachaItem[], rarity: number): number[] => {
    const distances: number[] = [];
    let lastIndex = -1;
    for (let i = 0; i < items.length; i++) {
        if (items[i].rarity === rarity) {
            if (lastIndex !== -1) {
                distances.push(i - lastIndex);
            }
            lastIndex = i;
        }
    }
    if (lastIndex !== -1) {
        distances.push(items.length - lastIndex);
    }
    return distances;
};


const getConveneName = (convene: Convene) => {
    switch (convene) {
        case Convene.EventCharacter:
            return '角色活动';
        case Convene.EventWeapon:
            return '武器活动';
        case Convene.PermanentCharacter:
            return '角色常驻';
        case Convene.PermanentWeapon:
            return '武器常驻';
        case Convene.Beginner:
            return '新手';
        case Convene.BeginnerSelected:
            return '新手自选';
        case Convene.BeginnerGiveBackSelected:
            return '新手自选(感恩回馈)';
        default:
            return convene;
    }
};

export default function GachaCard(props: GachaCardProps) {
    const { convene, items } = props.data;

    const fiveStarDistances = calculateDistance(items, 5);
    const fourStarDistances = calculateDistance(items, 4);

    const lastFiveStar = fiveStarDistances.length > 0 ? fiveStarDistances[0] : items.length;
    const lastFourStar = fourStarDistances.length > 0 ? fourStarDistances[0] : items.length;

    const fiveStarItems = items.filter(item => item.rarity === 5);
    const fourStarItems = items.filter(item => item.rarity === 4);

    const averageFiveStar = fiveStarItems.length ? (items.length / fiveStarItems.length).toFixed(2) : 'N/A';
    const averageFourStar = fourStarItems.length ? (items.length / fourStarItems.length).toFixed(2) : 'N/A';

    return (
        <Card title={getConveneName(convene)}>
            <Statistic title="总抽数" value={items.length} />
            <Statistic title="距离上个五星" value={lastFiveStar} />
            <Statistic title="距离上个四星" value={lastFourStar} />
            <Tabs defaultActiveKey="1">
                <TabPane tab="统计" key="1">
                    <div style={{ maxHeight: '150px', overflowY: 'auto' }}>
                        <Statistic title="五星平均抽数" value={averageFiveStar} />
                        <Statistic title="四星平均抽数" value={averageFourStar} />
                        <div className="gacha-avatar-container">
                            {fiveStarItems.map((item, index) => (
                                <GachaAvatarCard key={index} number={fiveStarDistances[index]} name={item.name} resourceId={item.id} />
                            ))}
                        </div>
                    </div>
                </TabPane>
                <TabPane tab="比例" key="2">
                    <div style={{ maxHeight: '150px', overflowY: 'auto' }}>
                        {/* 比例内容 */}
                    </div>
                </TabPane>
                <TabPane tab="预测" key="3">
                    <div style={{ maxHeight: '150px', overflowY: 'auto' }}>
                        {/* 预测内容 */}
                    </div>
                </TabPane>
            </Tabs>
        </Card>
    );
}