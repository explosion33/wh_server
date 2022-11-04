# wh_server
A custom webhook server

## Tech
### Back-end
Built with Rust using the Rocket and Tokio frameworks.

### Front-end
Built with vanilla JavaScript, Bootstrap 5, and HTML with Handlebars


## Features
* User specific webhooks
* login system with hashed and salted passwords

## Deployment
This server stack is built with Docker, to build ...

```docker build -t explosion33/wh .```

or

```docker pull explosion33/wh```

then

```docker run -p external:80 --name wh explosion33/wh```


## What's to come
* Database integration for more effecient route gathering
* User token to avoid having to store the user's password on the front-end
