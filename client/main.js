import pages from './pages/main'

window.onload = () => {
    const currentPage = document.body.dataset.page
    const page = pages[currentPage]
    if (!page) throw new Error(`Invalid page ${currentPage}!`)

    page() // initialize page
}
