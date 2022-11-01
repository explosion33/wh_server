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
    };

    xhr.send(JSON.stringify(data));

    
}

let show_new_hook = function(key, url) {
    console.log(key, url);

    let new_hook = document.createElement('div');
    new_hook.innerHTML = key + " --> " + url

    document.getElementById("hooks").appendChild(new_hook)
}