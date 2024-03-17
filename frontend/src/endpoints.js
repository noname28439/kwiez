/*
Beispiel Request für jeden implementierten Endpoint:

await useEndpoint("cq", {});
await useEndpoint("answer", {answer: "Das ist die Antwort"});

*/

import {authToken} from "./authentication.js";

export async function useEndpoint(method, data){
    let payload = [method, authToken, data]
    const response = await fetch("/api", {
        method: "POST",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify(payload)
    });
    try{
        return await response.json();
    }catch (SyntaxError){
        return undefined;
    }

}
