"use strict";
function fire_clicked(id) {
    window.external.invoke("{\"id\":\""+id+"\",\"value\":\"Clicked\"}");
}
function fire_value_changed(id) {
    window.external.invoke("{\"id\":\"" + id + "\",\"value\":{\"ValueChanged\":" + JSON.stringify($("#"+id).val()) + "}}");
}
function fire_created(id) {
    window.external.invoke("{\"id\":\""+id+"\",\"value\":\"Created\"}");
}