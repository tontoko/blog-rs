# blog.rs

## start local

```shell
  npm run dev
```

## dev in local with storyblok visual editor

### Install mkcert for creating a valid certificate (Mac OS):

```shell
  brew install mkcert
  mkcert -install
  mkcert localhost
```

### Then install and run the proxy

```shell
  npm install -g local-ssl-proxy
  local-ssl-proxy --source 3010 --target 8080 --cert localhost.pem --key localhost-key.pem
```

https is now running on port 3010 and forwarding requests to http 8080
