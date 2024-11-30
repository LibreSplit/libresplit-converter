# Stage 1: Build rust WASM pacakge.
FROM rust:latest AS wasm-builder

RUN cargo install wasm-pack

WORKDIR /app/wasm-lib

COPY ./wasm-lib .

RUN wasm-pack build --target web --out-dir /app/pkg

# Stage 2: Build React frontend.
FROM node:18 as react-builder

WORKDIR /app

COPY package.json package-lock.json ./
COPY ./public ./public
COPY ./src ./src
COPY tailwind.config.js tsconfig.json ./

COPY --from=wasm-builder /app/wasm-lib/pkg ./wasm-lib/pkg

RUN npm install

RUN npm run build

# Stage 3: Serve.
FROM nginx:alpine AS web-server

COPY --from=react-builder /app/build /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
