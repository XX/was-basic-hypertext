function set_page_header_height() {
    const header = document.querySelector('.page-header');
    if (header) {
        document.documentElement.style.setProperty('--page-header-height', header.offsetHeight + 'px');
    }
}

window.set_page_header_height = set_page_header_height;
