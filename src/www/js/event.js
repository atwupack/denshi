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

function fire_page_loaded() {
    safe_invoke("{\"id\":" + JSON.stringify('App') + ",\"value\":\"PageLoaded\"}");
}


/**
 * Fired if a tree node has been clicked.
 * @param node element clicked in tree.
 */
function fire_node_clicked(node) {
    safe_invoke("{\"id\":" + JSON.stringify(this.id) + ",\"value\":{\"ChildClicked\":" + JSON.stringify(node.id) + "}}");
}

function fire_node_expand(node) {
    safe_invoke("{\"id\":" + JSON.stringify(this.id) + ",\"value\":{\"NodeExpand\":" + JSON.stringify(node.id) + "}}");
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

function clear_node(idTree, idNode) {
    var tree = $('#'+idTree);
    var parentNode = $('#'+idNode);

    tree.data('treeview').clean(parentNode);
}

function add_tree_node(idTree, idParent, idNode, caption, hasChildren) {
    var tree = $('#'+idTree);
    var parentNode = $('#'+idParent);

    var new_node = tree.data('treeview').addTo(parentNode, {
        caption: caption
    });

    if (hasChildren) {
        tree.data('treeview').addTo(new_node, {
            caption: ''
        });
        tree.data('treeview').toggleNode(new_node);
    }

    new_node.id(idNode);
}