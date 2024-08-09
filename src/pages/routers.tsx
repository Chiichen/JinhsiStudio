
import HomePage from "./home";
import RouteErrorPage from "../components/base/base-route-error";
import GachaPage from "./gacha";
import { BaseErrorBoundary } from "../components/base/base-error-boundary";
import { useRoutes } from "react-router-dom";
import SettingsPage from "./settings";
import CharacterDataPage from "./data/character";
import GachaSettingPage from "./gacha/setting";

const routers = [
    {
        label: "Label-Home",
        path: "/",
        element: <HomePage></HomePage>,
        errorElement: <RouteErrorPage></RouteErrorPage>
    },
    {
        label: "Label-GachaTracker",
        path: "/gacha",
        element: <GachaPage></GachaPage>,
        errorElement: <RouteErrorPage></RouteErrorPage >
    },
    {
        label: "Label-GachaSetting",
        path: "/gacha/Setting",
        element: <GachaSettingPage></GachaSettingPage>,
        errorElement: <RouteErrorPage></RouteErrorPage >
    },
    {
        label: "Label-Settings",
        path: "/settings",
        element: <SettingsPage></SettingsPage>,
        errorElement: <RouteErrorPage></RouteErrorPage >
    },
    {
        label: "Label-Data-Character",
        path: "/data/character",
        element: <CharacterDataPage></CharacterDataPage>,
        errorElement: <RouteErrorPage></RouteErrorPage >
    }
].map((router) => ({
    ...router,
    element: (
        <BaseErrorBoundary key={router.label}>{router.element}</BaseErrorBoundary>
    ),
}))

const AppRoutes = () => {
    const element = useRoutes(routers);
    return element;
};

export default AppRoutes;