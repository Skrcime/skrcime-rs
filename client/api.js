const credentials = "same-origin";

const fetchJson = url =>
    fetch(url, { credentials }).then(res =>
        res.json().then(json => (res.ok ? json : Promise.reject(json)))
    );

const postJson = (url, payload) =>
    fetch(url, {
        method: "POST",
        headers: new Headers({ "Content-Type": "application/json" }),
        body: JSON.stringify(payload),
        credentials
    });

export default {
    login: payload => postJson("/api/session", payload),
    me: () => fetchJson("/api/users/me")
};
