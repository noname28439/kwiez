//example: await useEndpoint("answer", {"payload": "this", "name": "Kevon"})

export async function useEndpoint(method, data){
    let payload = [method, data]
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