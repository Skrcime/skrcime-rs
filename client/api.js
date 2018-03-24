const credentials = "same-origin";
const headers = new Headers({ "Content-Type": "application/json" });

const request = {
    get: url =>
        fetch(url, { credentials }).then(res =>
            res.json().then(json => (res.ok ? json : Promise.reject(json)))
        ),
    post: (url, payload) =>
        fetch(url, {
            method: "POST",
            body: JSON.stringify(payload),
            headers,
            credentials
        }).then(res =>
            res.json().then(json => (res.ok ? json : Promise.reject(json)))
        ),
    delete: (url) =>
        fetch(url, { method: "DELETE", credentials })
};

export default {
    login: payload => request.post("/api/session", payload),
    logout: () => request.delete("/api/session"),
    register: payload => request.post("/api/users", payload),
    newUrl: payload => request.post("/api/urls", payload),
    me: () => request.get("/api/users/me")
};
