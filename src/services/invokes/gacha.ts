import { GachaLog } from "@/models/gacha/gacha-log";
import { invoke } from "@tauri-apps/api/core";

export async function getGachaLogFromUrl(url:string):Promise<GachaLog[]|void> {
  console.log("getGachaLogFromUrl",url);
  return invoke<GachaLog[]|void>("get_gachalog_from_url",{url}).catch((error) => console.error(error));
}
export async function getGachaLogFromLocal(path:string|null):Promise<GachaLog[]|void> {
  console.log("getGachaLogFromUrl",path);
  return invoke<GachaLog[]|void>("get_gachalog_from_local",{path}).catch((error) => console.error(error));
}