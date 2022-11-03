let login = function(username, password) {
    sessionStorage.setItem("username", username);
    sessionStorage.setItem("password", password);
    window.location.href = "/view";
}

window.onload = function() {
    let username = sessionStorage.getItem("username");
    let password = sessionStorage.getItem("password");

    if (username == null || password == null || username == "" || password == "") {
        console.log("no user found");
    }
    else {
        window.location.href = "/view";
    }
}