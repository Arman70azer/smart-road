# Étape 1 : Construction de l'application
# FROM rust:latest AS builder
FROM rust:1.67-buster AS builder

# Définir le répertoire de travail
WORKDIR /usr/src/smart_road

# Copier les fichiers de configuration
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs

# Construire les dépendances du projet
RUN cargo build --release

# Copier le reste du code source
COPY . .

# Construire le projet
RUN cargo build --release

# Étape 2 : Création de l'image finale
# FROM debian:bullseye-slim
FROM debian:latest

# Installer les bibliothèques nécessaires
RUN apt-get update && \
    apt-get install -y \
    build-essential \
    libssl-dev \
    libsdl2-dev \
    libsdl2-image-dev \
    libsdl2-ttf-dev \
    && rm -rf /var/lib/apt/lists/*

# Copier l'application construite depuis l'étape précédente
COPY --from=builder /usr/src/smart_road/target/release/smart_road /usr/local/bin/smart_road_image

# Définir la commande par défaut
CMD ["smart_road"]


# Exposer le port si nécessaire (par exemple pour une application serveur)
# EXPOSE 8080


# PROCÉDURE DE DÉMARRAGE:

# créer l'image docker à partir du terminal: 
# - sudo docker build -t smart_road_image .

# créer l'image docker à partir du terminal:
# - docker container run --detach --name smart_road 

# Arrêt du docker: 
# - Docker stop smart_road

# Démarrage du docker:
# - Docker start smart_road

# Effacé le docker:
# - Docker rm smart_road

# Effacer l’image:
# - Docker rmi smart_road

# liste des images forum:
# - docker images smart_road_image

# liste des conteneurs:
# - docker ps -a