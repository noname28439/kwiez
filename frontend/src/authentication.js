export var authToken = localStorage.getItem("authToken")==null ? generateNewAuthToken() : localStorage.getItem("authToken");

function generateNewAuthToken(){
    let nextToken = makeid(64);
    localStorage.setItem("authToken", nextToken);
    return nextToken;
}


function makeid(length) {
    let result = '';
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    while (result.length < length) result += characters.charAt(Math.floor(Math.random() * characters.length))
    return result;
}