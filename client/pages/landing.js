import api from "../api";

const onNewUrl = e => {
    e.preventDefault();

    const target = document.getElementById("url-input").value
    if (!target) return

    api
        .newUrl({ target })
        .then(res => {
            alert(JSON.stringify(res, null, "\t"));
        })
        .catch(err => {
            alert(JSON.stringify(err, null, "\t"));
        });
};

export default function init() {
    document.getElementById("url-action").addEventListener("click", onNewUrl);
}
