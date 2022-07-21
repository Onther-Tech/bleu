# Bleu

**BLockchain Explorer sUite**

Bleu is a collection of tools for building blockchain explorer.

## preparatory work

To get started with Bleu Blockexplorer, you need the following elements:

1. RUST
   ```
   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Nodejs and yarn

   ```shell
   $ sudo apt-get update
   $ sudo apt-get install -y build-essential
   $ sudo apt-get install curl
   $ curl -sL https://deb.nodesource.com/setup_10.x | sudo -E bash --
   $ sudo apt-get install -y node.js

   $ npm install -g yarn
   ```

3. Postgresql
   ```
   sudo apt-get install postgresql postgresql-contrib
   ```

## Execution order

- The data collected by bleu-daemon is utilized by bleu-server and bleu-app, so you need to run `bleu-daemon -> bleu-server -> bleu-app` in the order.
- For the configuration and execution method of each package, refer to the `README` for each package.

## License

[AGPL-3.0](https://github.com/turnpike/bleu/blob/main/LICENSE)
