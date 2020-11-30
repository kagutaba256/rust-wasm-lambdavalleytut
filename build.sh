#!/bin/bash

rm -rf www/node_modules
wasm-pack build
cd www
yarn install
yarn start