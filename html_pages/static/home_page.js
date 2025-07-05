// html_pages/static/home_page.js
console.log("Home page JavaScript loaded!");
document.addEventListener('DOMContentLoaded', () => {
    const header = document.querySelector('h1');
    if (header) {
        header.style.color = 'purple';
    }
});