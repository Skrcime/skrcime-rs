import api from '../api'

const onMe = (e) => {
    e.preventDefault()

    api.me()
        .then((res) => console.log(res))
        .catch((err) => console.error('Error', err))
}

export default function init() {
    document.getElementById('me').addEventListener('click', onMe)
}
