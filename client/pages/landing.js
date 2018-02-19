import api from "../api";

const onMe = e => {
    e.preventDefault();

    api
        .me()
        .then(res => {
            alert(JSON.stringify(res, null, "\t"));
        })
        .catch(err => {
            alert(JSON.stringify(err, null, "\t"));
        });
};

export default function init() {
    document.getElementById("me").addEventListener("click", onMe);
}
