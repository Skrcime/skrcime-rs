import api from "./api";

const onLogout = e => {
    e.preventDefault();

    api
        .logout()
        .then(() => {
            window.location = "/";
        })
};

export default function init(hasSession) {
  if (!hasSession) return;

  document.getElementById('logout').addEventListener("click", onLogout);
}
