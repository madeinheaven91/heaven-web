FROM node:lts-alpine

# Install `serve` instead of `http-server`
RUN npm install -g serve

WORKDIR /app

COPY package*.json ./
RUN npm install

COPY . .

RUN npm run build

EXPOSE 8080

CMD [ "serve", "-s", "dist" ]
