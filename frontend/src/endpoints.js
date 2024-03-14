//example: await useEndpoint("answer", {"payload": "this", "name": "Kevon"})

export async function useEndpoint(route, payload){
    const response = await fetch("/"+route, {
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