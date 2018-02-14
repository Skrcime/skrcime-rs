
const onLogin = (e) => {
    e.preventDefault()

    const { email, password } = e.target.elements
    window.common.api.login({ email: email.value, password: password.value})
        .then((res) => {
            if (res.ok) window.location = '/'
            else alert(`${res.status} ${res.statusText}`)
        })
        .catch((err) => console.error('Error', err))
}

window.onload = () => {
    document.forms['login_form'].addEventListener('submit', onLogin)
}
