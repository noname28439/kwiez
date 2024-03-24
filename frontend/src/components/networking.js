import {useEndpoint} from "../endpoints.js";
import {stats, ranking, cq} from "./stores.js";

export async function sync() {
    let statsRes = await useEndpoint("stats", {});
    let rankingRes = await useEndpoint("ranking", {})
    let cqRes = await useEndpoint("cq", {})

    stats.set(statsRes);
    ranking.set(rankingRes);
    cq.set(cqRes)

    return({stats, ranking, cq})
}