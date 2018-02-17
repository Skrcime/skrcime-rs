import api from "../api";

const onLogin = e => {
    e.preventDefault();

    const { email, password } = e.target.elements;
    api
        .login({ email: email.value, password: password.value })
        .then(res => {
            if (res.ok) window.location = "/";
            else alert(`${res.status} ${res.statusText}`);
        })
        .catch(err => console.error("Error", err));
};

export default function init() {
    document.forms["login_form"].addEventListener("submit", onLogin);
}
