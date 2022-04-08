(function() {

    let urlPreviews = document.querySelectorAll('.url-preview');
    for (let preview of urlPreviews) {
        preview.onclick = (event) => {
            // Ignore the event if the user click the A tag.
            if (event.target.tagName === 'A') return;

            let a = preview.lastElementChild;
            if (a && a.href) {
                window.open(a.href);
            }
        };
    }

    mediumZoom(document.querySelectorAll('.prose p>img'), {
        margin: 24,
        background: '#000C',
    });
})();