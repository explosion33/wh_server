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

let remove_hook = function(key) {
    let xhr = new XMLHttpRequest();
    xhr.open("POST", "/delete");

    xhr.setRequestHeader("Accept", "application/json");
    xhr.setRequestHeader("Content-Type", "application/json");

    xhr.onload = function() {
        get_routes(sessionStorage.getItem("username"), sessionStorage.getItem("password"));
    };

    let data = {
        "key": key,
        "username": sessionStorage.getItem("username"),
        "password": sessionStorage.getItem("password") 
    };

    xhr.send(JSON.stringify(data));
}

let show_new_hook = function(key, url) {
    console.log(key, url);

    let new_hook = document.createElement('div');
    new_hook.innerHTML = window.location.origin + "/hook/" + key + " --> " + url

    let delete_btn = document.createElement('button');
    delete_btn.innerText = "Delete"
    delete_btn.onclick = function() {
        remove_hook(key);
    }

    new_hook.innerText += "     ";
    new_hook.appendChild(delete_btn);

    document.getElementById("hooks").appendChild(new_hook);
}

let login = function(username, password) {
    sessionStorage.setItem("username", username);
    sessionStorage.setItem("password", password);
    show_add();
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
    document.getElementById("add").style.display = "none";
    document.getElementById("login").style.display = "block";
    document.getElementById("username").innerText = "";
}

let show_add = function() {
    document.getElementById("add").style.display = "block";
    document.getElementById("login").style.display = "none";
    document.getElementById("username").innerText = sessionStorage.getItem("username");
}

window.onload = function() {
    let username = sessionStorage.getItem("username");
    let password = sessionStorage.getItem("password");

    if (username == null || password == null) {
        console.log("no user found");

        show_login()
    }
    else {
        get_routes(username, password);
        console.log(username);
        console.log(password);
        show_add()
    }
}