# 📋 Prérequis

**Token GitHub** : Pour générer un token github, vous devez :

1. Aller dans les paramètres de votre compte GitHub personnel.
2. Cliquer sur "Developer settings" dans le menu latéral, tout en bas à gauche.
3. Sélectionner "Personal access tokens" puis "Fine-grained tokens".
4. Cliquer sur "Generate new token" et donner un nom à votre token.
5. Sélectionner "Public repositories" dans la section "Repository access".
6. Cliquer sur "Generate token" et copier le token généré.
7. Coller le token dans le fichier `.env` à la racine du projet.

# Utilisation

1. Clonez le dépôt.
2. Construisez l'application avec `cargo build --release`.
3. Créez un fichier `.env` dans le dossier où se trouve l'application.
4. Remplissez le `.env` :
   ```
   GITHUB_TOKEN=your_token_here
   DL_FOLDER_PATH=/path/to/download/folder
   ORGANIZATION_TO_FETCH=your_organization
   ```
5. Exécutez l'application depuis un terminal.
