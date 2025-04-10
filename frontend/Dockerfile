# FROM node:23-alpine3.20
#
#
# WORKDIR /app
#
# COPY package*.json ./
# RUN npm install
# RUN npm install -g http-server
#
# COPY . .
#
# RUN npm run build
#
# EXPOSE 8080
#
# CMD [ "http-server", "dist" ]

# Build stage
FROM node:lts-alpine as build-stage

WORKDIR /app
COPY package*.json ./
RUN npm install

COPY . .
RUN npm run build

# Production stage
FROM nginx:alpine as production-stage

# Remove default nginx config
RUN rm /etc/nginx/conf.d/default.conf

# Copy custom nginx config
COPY nginx.conf /etc/nginx/conf.d

# Copy built files to nginx public folder
COPY --from=build-stage /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
