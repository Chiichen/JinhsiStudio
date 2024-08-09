
import { useTranslation } from "react-i18next"
import { SettingItem, SettingList } from "./mod/setting-component"
import { Select } from "antd";
import { supportedLanguages } from "@/services/i18n";


export default function AppSetting() {
    const { t, i18n } = useTranslation()
    const handleChange = (value: string) => {
        i18n.changeLanguage(value)
    };

    return <SettingList title={t("App-Settings")}>

        <SettingItem label={t("Language")}>
            <Select
                defaultValue={i18n.language}
                style={{ width: 120 }}
                onChange={handleChange}
                options={supportedLanguages}
            />
        </SettingItem>
    </SettingList>
}