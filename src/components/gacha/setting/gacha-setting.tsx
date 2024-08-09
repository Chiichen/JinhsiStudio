import { SettingItem, SettingList } from "@/components/setttings/mod/setting-component"
import { Input } from "antd"
import { useTranslation } from "react-i18next"

export default function GachaSetting() {
    const { t } = useTranslation()

    return <SettingList title={t("Gacha-Settings")}>
        <SettingItem label={t("Label-Gacha-Url")}>
        </SettingItem>
        <Input />
    </SettingList>
}