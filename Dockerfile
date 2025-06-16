#####################################################################################################
FROM rust:1.86 AS build-backend

WORKDIR /app

# Pré-compila as dependências e mantém elas em cache pra diminuir o tempo de compilação
COPY Cargo.lock Cargo.toml rust-toolchain.toml ./
COPY config/ ./config
COPY sessions/ ./sessions/

# É necessário para manter as dependências em cache. O Cargo não possui um meio de compilar
# somente as dependências para evitar recompilar tudo a cada alteração no código fonte,
# e esta é uma gambiarra pra atingir algo próximo disso.
#
# Veja: https://github.com/rust-lang/cargo/issues/2644#issuecomment-2335499312
RUN \
    mkdir -v src && \
    echo 'fn main() {}' > src/main.rs && \
    cargo build --release && \
    rm -Rvf src

# Copia o resto da aplicação e compila em uma segunda camada, evitando a invalidação do cache
# das dependências acima
COPY . .
RUN cargo build --release --locked --features=dockerimgb

#####################################################################################################
FROM node:22-slim AS build-frontend

WORKDIR /app

# Instala as dependências em outra camada para mantê-las em cache
COPY package.json package-lock.json ./
RUN npm ci

# Builda o front-end em outra camada para evitar invalidar o cache das dependências à toa
COPY www/ ./www/
COPY public/ ./public/
COPY tsconfig.json vite.config.ts ./
RUN npm run build

#####################################################################################################
FROM ubuntu:22.10 AS final

WORKDIR /app

COPY --from=build-backend /app/target/release/projetario .
COPY --from=build-frontend /app/public/ ./public/
COPY --from=build-frontend /app/dist/ ./dist/
COPY www/root.hbs ./www/root.hbs

EXPOSE 80

# Cria um usuário somente para rodar o sistema, de modo que ele não tenha privilégios de super usuário.
# Ele ainda precisa ter permissão pra manusear os arquivos dentro de /app, no entanto.
RUN useradd -m projetario
RUN chown -R projetario:projetario /app
RUN chmod 755 /app

USER projetario

ENTRYPOINT [ "./projetario" ]
