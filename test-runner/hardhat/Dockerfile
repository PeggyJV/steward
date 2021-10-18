FROM node:14-alpine3.13

RUN apk update
RUN apk add --no-cache git

COPY package.json package.json

RUN yarn install --production=false
RUN npm config set user 0

COPY . .

EXPOSE 8545
