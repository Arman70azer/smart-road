# Étape 1 : Construction
FROM rust:latest as builder

# Créer un répertoire de travail
WORKDIR /usr/src/myapp

# Copier les fichiers Cargo.toml et Cargo.lock (si présents) pour la construction des dépendances
COPY Cargo.toml Cargo.lock ./

# Créer un fichier pour les dépendances
RUN mkdir src && echo 'fn main() {}' > src/main.rs

# Installer les dépendances
RUN cargo build --release

# Copier le code source du projet
COPY . .

# Construire l'application en mode release
RUN cargo build --release

# Étape 2 : Création de l'image finale
FROM alpine:latest

# Installer les bibliothèques nécessaires
RUN apk add --no-cache libssl1.1

# Créer un répertoire de travail
WORKDIR /usr/local/bin

# Copier l'exécutable depuis l'image de construction
COPY --from=builder /usr/src/myapp/target/release/myapp ./smart_road

# Définir le point d'entrée du conteneur
ENTRYPOINT ["./smart_road"]

# Exposer le port si nécessaire (par exemple pour une application serveur)
# EXPOSE 8080
