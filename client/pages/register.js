import api from "../api";

const onRegister = e => {
    e.preventDefault();

    const { name, email, password } = e.target.elements;
    api
        .register({
            name: name.value,
            email: email.value,
            password: password.value
        })
        .then(res => {
            window.location = "/prijava";
        })
        .catch(err => {
            alert(JSON.stringify(err));
        });
};

export default function init() {
    document.forms["register_form"].addEventListener("submit", onRegister);
}
