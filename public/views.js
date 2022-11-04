let register_hook = function(url) {
    let xhr = new XMLHttpRequest();
    xhr.open("POST", "/hook");

    xhr.setRequestHeader("Accept", "application/json");
    xhr.setRequestHeader("Content-Type", "application/json");

    xhr.onload = function() {
        show_new_hook(xhr.response, url)
    };

    let data = {
        "url": url,
        "username": sessionStorage.getItem("username"),
        "password": sessionStorage.getItem("password") 
    };

    xhr.send(JSON.stringify(data));

    
}

let remove_hook = function(key, do_reload=false) {
    let xhr = new XMLHttpRequest();
    xhr.open("POST", "/delete");

    xhr.setRequestHeader("Accept", "application/json");
    xhr.setRequestHeader("Content-Type", "application/json");

    if (do_reload) {
        xhr.onload = function() {
            get_routes(sessionStorage.getItem("username"), sessionStorage.getItem("password"));
            document.getElementById("delete").hidden = true;
        };
    }

    let data = {
        "key": key,
        "username": sessionStorage.getItem("username"),
        "password": sessionStorage.getItem("password") 
    };

    xhr.send(JSON.stringify(data));
}

let remove_hooks = function(keys, do_reload=false) {
    let xhr = new XMLHttpRequest();
    xhr.open("POST", "/delete_multiple");

    xhr.setRequestHeader("Accept", "application/json");
    xhr.setRequestHeader("Content-Type", "application/json");

    if (do_reload) {
        xhr.onload = function() {
            get_routes(sessionStorage.getItem("username"), sessionStorage.getItem("password"));
            document.getElementById("delete").hidden = true;
        };
    }

    let data = {
        "keys": keys,
        "username": sessionStorage.getItem("username"),
        "password": sessionStorage.getItem("password") 
    };

    xhr.send(JSON.stringify(data));
}

let show_new_hook = function(key, url) {
    console.log(key, url);

    let element =
    `
    <div class="row">
        <div class="col-sm-5">
            ${window.location.origin + "/hook/" + key}
        </div>
        <div class="col-sm-5">
            ${url}               
        </div>
        <div class="col-sm-2 text-center">
            <input type="checkbox" id="${key}" name="select" onclick="check_for_delete()">
        </div>
    </div>
    `

    //<button type="button" class="btn btn-danger" onclick="remove_hook(${key})">Delete</button>

    document.getElementById("hooks").innerHTML += element;

    document.getElementById("select_all").checked = false;
}

let get_routes = function(username, password) {
    let xhr = new XMLHttpRequest();
    xhr.open("POST", "/user_hooks");

    xhr.setRequestHeader("Accept", "application/json");
    xhr.setRequestHeader("Content-Type", "application/json");

    xhr.onload = function() {
        document.getElementById("hooks").innerText = ""
        JSON.parse(xhr.response).forEach(route => {
            show_new_hook(route.key, route.url);
        });
    };

    let data = {
        "username": username,
        "password": password 
    };

    xhr.send(JSON.stringify(data));
}

let show_login = function() {
    sessionStorage.removeItem("username");
    sessionStorage.removeItem("password");
    window.location.href = "/"
}

let check_for_delete = function() {
    let btn = document.getElementById("delete")
    
    let found = false;
    document.getElementsByName("select").forEach(element => {
        if (element.checked == true) {
            found = true;
            btn.hidden = false;
            return;
        }
        else {
            document.getElementById("select_all").checked = false;
        }
    });

    if (!found) {
        btn.hidden = true;
    }
} 

let delete_all = function() {
    let boxes = document.getElementsByName("select");
    
    let checked = [];

    boxes.forEach(element => {
        if (element.checked) {
            checked.push(element.id);
        }
    });

    if (checked.length == 0) {
        
    }

    else if (checked.length == 1) {
        remove_hook(checked[0], true)
    }

    else {
        remove_hooks(checked, true);
    }
}

let select_all = function() {
    let state = document.getElementById("select_all").checked;

    document.getElementsByName("select").forEach(element => {
        element.checked = state;
    });

    check_for_delete();


}

window.onload = function() {
    let username = sessionStorage.getItem("username");
    let password = sessionStorage.getItem("password");

    if (username == null || password == null || username == "" || password == "") {
        show_login();
    }
    else {
        document.getElementById("title").innerText = username;
        get_routes(username, password);
    }
}