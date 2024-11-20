FROM node:20

WORKDIR /Project-Catan

COPY package.json ./

RUN npm install

COPY . .

ENV PORT=5000

EXPOSE 5000

CMD [ "npm", "run", "build:prod"]