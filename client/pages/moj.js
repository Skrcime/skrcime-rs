import api from "../api";

const fetchUrls = () => {
    api
        .myUrls()
        .then(res => {
            const listDom = document.getElementById("list-urls")
            res.forEach(url => {
                var li = document.createElement("li")
                li.innerText = url.target
                listDom.appendChild(li)
            })
        })
        .catch(err => {
            alert(JSON.stringify(err));
        })
}

export default function init() {
    fetchUrls()
}
