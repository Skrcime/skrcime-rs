import api from "../api";

const onNewUrl = e => {
    e.preventDefault();

    api
        .newUrl({ target: "https://google.com" })
        .then(res => {
            alert(JSON.stringify(res, null, "\t"));
        })
        .catch(err => {
            alert(JSON.stringify(err, null, "\t"));
        });
};

export default function init() {
    document.getElementById("new-url").addEventListener("click", onNewUrl);
}
