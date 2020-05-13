"use strict";
function fire_clicked(id) {
    safe_invoke("{\"id\":\"" + id + "\",\"value\":\"Clicked\"}");
}
function fire_value_changed(id) {
    safe_invoke("{\"id\":\"" + id + "\",\"value\":{\"ValueChanged\":" + JSON.stringify($("#" + id).val()) + "}}");
}
function fire_created(id) {
    safe_invoke("{\"id\":\"" + id + "\",\"value\":\"Created\"}");
}

function fire_node_clicked(node) {
    safe_invoke("{\"id\":\"" + this.id + "\",\"value\":{\"ChildClicked\":" + JSON.stringify(node.id) + "}}");
}

function safe_invoke(str) {
    try {
        window.external.invoke(str);
    }
    catch (e) {

    }
}