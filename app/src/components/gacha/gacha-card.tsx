import { getConveneLabel } from "@/models/gacha/convene";
import { GachaItem } from "@/models/gacha/gacha-item";
import { GachaLog } from "@/models/gacha/gacha-log";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import GachaAvatarCard from "./gacha-avatar-card";
import { GachaStatisticPullTitle } from "./statistic/gacha-statistic-pull-title";
import { SingleSelect } from "../ui/select";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/base/card";
import { Separator } from "@/components/ui/base/separator";

interface GachaCardProps {
  data: GachaLog[];
}

function calculateDistance(items: GachaItem[], rarity: number): number[] {
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
}

function sum(arr: number[]): number {
  if (arr.length !== 0) {
    return arr.reduce((prev, cur) => prev + cur);
  } else {
    return 0;
  }
}

export default function GachaCard(props: GachaCardProps) {
  const { t } = useTranslation("gacha");
  const [currentGacha, setCurrentGacha] = useState<GachaLog>(props.data[0]);

  const fiveStarDistances = calculateDistance(currentGacha.items, 5);
  const fourStarDistances = calculateDistance(currentGacha.items, 4);

  const lastFiveStar = currentGacha.items.length - sum(fiveStarDistances);
  const lastFourStar = currentGacha.items.length - sum(fourStarDistances);

  const fiveStarItems = currentGacha.items.filter((item) => item.rarity === 5);

  const averageFiveStar = fiveStarDistances.length
    ? (sum(fiveStarDistances) / fiveStarDistances.length).toFixed(2)
    : "N/A";
  const averageFourStar = fourStarDistances.length
    ? (sum(fourStarDistances) / fourStarDistances.length).toFixed(2)
    : "N/A";

  const titleItems = props.data.map((value: GachaLog, index: number) => {
    return {
      value: index.toString(),
      label: t("common." + getConveneLabel(value.convene)),
    };
  });

  {
    return (
      <Card className="min-h-full max-h-[calc(100vh-2rem)] bg-card mx-4 overflow-y-auto overflow-x-hidden">
        <CardHeader className="pb-2">
          <div className="flex justify-between items-center">
            <CardTitle className="text-xl font-medium">
              {t("common." + getConveneLabel(currentGacha.convene))}
            </CardTitle>
            <div className="w-fit min-w-32">
              <SingleSelect
                options={titleItems}
                onValueChange={(value: string) =>
                  setCurrentGacha(props.data[Number.parseInt(value)])
                }
                defaultValue={"0"}
              />
            </div>
          </div>
        </CardHeader>
        <CardContent>
          {/* 总抽数展示 */}
          <div className="flex justify-center">
            <GachaStatisticPullTitle pullCount={currentGacha.items.length} />
          </div>

          {/* 统计数据卡片 */}
          <div className="mt-6 grid grid-cols-2 gap-6">
            {/* 五星统计 */}
            <div className="rounded-xl border bg-background p-6 shadow-sm">
              <div className="relative">
                <h3 className="text-lg font-semibold text-foreground">
                  {t("common.Label-FiveStarStats")}
                </h3>
                <div className="mt-4 grid grid-cols-2 gap-4">
                  <div>
                    <p className="text-sm font-medium text-muted-foreground">
                      {t("common.Label-LastPull")} 5★
                    </p>
                    <p
                      className="mt-1 text-3xl font-bold tracking-tight"
                      role="gacha-last-five-star-count"
                    >
                      {lastFiveStar}
                    </p>
                  </div>
                  <div>
                    <p className="text-sm font-medium text-muted-foreground">
                      {t("common.Label-AvgPulls")}
                    </p>
                    <p
                      className="mt-1 text-3xl font-bold tracking-tight"
                      role="gacha-average-five-star-count"
                    >
                      {averageFiveStar}
                    </p>
                  </div>
                </div>
              </div>
            </div>

            {/* 四星统计 */}
            <div className="rounded-xl border bg-background p-6 shadow-sm">
              <div className="relative">
                <h3 className="text-lg font-semibold text-foreground">
                  {t("common.Label-FourStarStats")}
                </h3>
                <div className="mt-4 grid grid-cols-2 gap-4">
                  <div>
                    <p className="text-sm font-medium text-muted-foreground">
                      {t("common.Label-LastPull")} 4★
                    </p>
                    <p
                      className="mt-1 text-3xl font-bold tracking-tight"
                      role="gacha-last-four-star-count"
                    >
                      {lastFourStar}
                    </p>
                  </div>
                  <div>
                    <p className="text-sm font-medium text-muted-foreground">
                      {t("common.Label-AvgPulls")}
                    </p>
                    <p
                      className="mt-1 text-3xl font-bold tracking-tight"
                      role="gacha-average-four-star-count"
                    >
                      {averageFourStar}
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <Separator className="my-6" />

          {/* 五星角色展示 */}
          <div>
            <h3 className="text-lg font-medium mb-2">
              {t("common.Label-FiveStarHistory")}
            </h3>
            <div className="flex flex-wrap gap-2">
              {fiveStarItems.map((item, index) => (
                <GachaAvatarCard
                  key={index}
                  number={fiveStarDistances[index]}
                  name={item.name}
                  resourceId={item.id}
                />
              ))}
            </div>
          </div>
        </CardContent>
      </Card>
    );
  }
}
