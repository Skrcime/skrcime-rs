
const fetchWrapper = (url) =>
    fetch(url, { credentials: 'same-origin' })
        .then((res) => res.json()
            .then((json) => res.ok ? json : Promise.reject(json)))

export default {
    login: ({ email, password }) =>
        fetch('/api/session', {
            method: 'POST',
            body: JSON.stringify({ email, password }), 
            headers: new Headers({ 'Content-Type': 'application/json' }),
            credentials: 'same-origin'
        }),
    me: () => fetchWrapper('/api/users/me')
}
