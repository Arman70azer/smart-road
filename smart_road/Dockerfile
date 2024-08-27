# Étape 1 : Utiliser l'image officielle Rust pour la compilation
FROM rust:1.72 as builder

# Étape 2 : Définir le répertoire de travail dans le conteneur
WORKDIR /usr/src/smart-road

# Étape 3 : Copier les fichiers Cargo.toml et Cargo.lock pour profiter du cache
COPY Cargo.toml Cargo.lock ./

# Étape 4 : Télécharger les dépendances du projet
RUN cargo fetch

# Étape 5 : Copier le reste du projet dans le conteneur
COPY . .

# Étape 6 : Compiler le projet en mode release
RUN cargo build --release

# Étape 7 : Utiliser une image plus légère pour exécuter l'application
FROM debian:buster-slim

# Étape 8 : Copier l'exécutable compilé depuis l'image de build
COPY --from=builder /usr/src/smart-road/target/release/smart-road /usr/local/bin/smart-road

# Étape 9 : Exposer le port sur lequel votre application écoute (ajustez si nécessaire)
EXPOSE 8080

# Étape 10 : Définir la commande pour démarrer l'application
CMD ["smart-road"]
