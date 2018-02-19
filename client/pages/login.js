import api from "../api";

const onLogin = e => {
    e.preventDefault();

    const { email, password } = e.target.elements;
    api
        .login({ email: email.value, password: password.value })
        .then(res => {
            window.location = "/";
        })
        .catch(err => {
            alert(JSON.stringify(err, null, "\t"));
        });
};

export default function init() {
    document.forms["login_form"].addEventListener("submit", onLogin);
}
