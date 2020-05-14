"use strict";
function fire_clicked(id) {
    safe_invoke("{\"id\":" + JSON.stringify(id) + ",\"value\":\"Clicked\"}");
}

function fire_value_changed(id) {
    safe_invoke("{\"id\":" + JSON.stringify(id) + ",\"value\":{\"ValueChanged\":" + JSON.stringify($("#" + id).val()) + "}}");
}

/**
 * Fired if a component has been created.
 */
function fire_created() {
    safe_invoke("{\"id\":" + JSON.stringify(this.id) + ",\"value\":\"Created\"}");
}

/**
 * Fired if a tree node has been clicked.
 * @param node element clicked in tree.
 */
function fire_node_clicked(node) {
    safe_invoke("{\"id\":" + JSON.stringify(this.id) + ",\"value\":{\"ChildClicked\":" + JSON.stringify(node.id) + "}}");
}

/**
 * Utility function to call invoke w/o errors in a browser.
 * @param str string to be sent as event.
 */
function safe_invoke(str) {
    try {
        window.external.invoke(str);
    }
    catch (e) {

    }
}