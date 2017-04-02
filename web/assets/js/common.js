// ponemos como active los links de la navbar que coincidan con la sección en la que estamos
(function put_correct_navitem_active() {
    var re = /https?:\/\/.*?(\/.*?)(?:\/|$)/;
    var current_section = re.exec(document.URL)[1];
    var navlinks = document.getElementsByClassName("nav-link");

    for (var i = 0; i < navlinks.length; i++) {
        if (re.exec(navlinks[i].href)[1] == current_section) {
            navlinks[i].className += " active";
            return;
        }
    }
})();

// put_correct_navitem_active();