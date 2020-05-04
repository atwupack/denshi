"use strict";
function fire_clicked(id) {
    window.external.invoke("{\"id\":\""+id+"\",\"value\":\"Clicked\"}");
}
function fire_value_changed(id) {
    var elem = document.getElementById(id);
    if (elem) {
        window.external.invoke("{\"id\":\"" + id + "\",\"value\":{\"ValueChanged\":\"" + elem.value + "\"}}");
    }
}
