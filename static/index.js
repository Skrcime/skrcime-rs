
const onMe = (e) => {
    e.preventDefault()

    window.common.api.me()
        .then((res) => console.log(res))
        .catch((err) => console.error('Error', err))
}

window.onload = () => {
    document.getElementById('me').addEventListener('click', onMe)
}
