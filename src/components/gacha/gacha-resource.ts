import i18next from "i18next"

import jinhsi from '@/assets/resonators/jinhsi.webp';
import changli from '@/assets/resonators/changli.webp';

enum Character{
    Jinhsi = 1304,
    Changli = 1205
}

export function matchResource(name: string, resourceId: number): string | null {
    //TODO match resource with resourceId instead of name
    switch(resourceId){
        case Character.Changli:
            return changli
        case Character.Jinhsi:
            return jinhsi
    }
    switch (name) {
        case i18next.t('resource:Character.Changli'):
            return changli
        case i18next.t('resource:Character.Jinhsi'):
            return jinhsi
        default:
            console.error("Unmatched resource: ",name,"with id: ",resourceId)
            return null
    }
}