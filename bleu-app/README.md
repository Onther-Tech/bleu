This project was bootstrapped with [Create React App](https://github.com/facebook/create-react-app).

## Getting Started

First, install npm packages:

```bash
yarn

or

yarn install
```

## Build

### Create local environment files

**for development**
nothing

**for rinkeby**

```
cp .env.rinkeby .env.local
```

**for mainnet**

```
cp .env.mainnet .env.local
```


### run

```
yarn start
```

### generate static web files

```
yarn build

ls -l build/
```

## Docker Image Build

**for rinkeby**

```
docker build -t tokamak-gateway-explorer-app . --build-arg NODE_ENV=rinkeby

docker run -p 3000:3000 tokamak-gateway-explorer-app
```
