export interface IGachaItem{
    id: number,
    rarity: number,
    name: string,
    date: string,
}

export class GachaItem implements IGachaItem{
    constructor(id:number,rariry:number,name:string,date:string){
        this.id = id;
        this.rarity = rariry;
        this.name = name;
        this.date = date;
    }
    id: number;
    rarity: number;
    name: string;
    date: string;
}