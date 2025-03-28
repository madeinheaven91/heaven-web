FROM node:23-alpine3.20


WORKDIR /app

COPY package*.json ./
RUN npm install
RUN npm install -g http-server

COPY . .

RUN npm run build

EXPOSE 8080

CMD [ "http-server", "dist" ]
