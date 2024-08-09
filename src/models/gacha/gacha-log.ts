
import { Convene } from "./convene";
import { GachaItem, IGachaItem } from "./gacha-item";

export interface IGachaLog{
    convene: Convene,
    items:IGachaItem[],
}
export class GachaLog implements IGachaLog{
    constructor(convene:Convene,items:GachaItem[]){
        this.convene = convene;
        this.items = items;
    }
    convene: Convene;
    items: GachaItem[];
}