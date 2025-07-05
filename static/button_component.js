document.addEventListener('DOMContentLoaded', () => {
    const button = document.querySelector('.my-button'); // Or use the dynamic ID
    if (button) {
        button.addEventListener('click', () => {
            alert('Button clicked from component JS!');
        });
    }
});