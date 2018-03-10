import header from "./header";
import pages from "./pages/main";

const hasSession = 'USER' in window

window.onload = () => {
    header(hasSession);

    const currentPage = document.body.dataset.page;
    const page = pages[currentPage];
    if (!page) throw new Error(`Invalid page ${currentPage}!`);

    page(hasSession); // initialize page
};
