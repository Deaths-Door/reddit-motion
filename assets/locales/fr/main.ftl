thanks = Merci d'utiliser cet outil ! N'hésitez pas à contribuer à ce projet sur GitHub !
questions = Si vous avez des questions, n'hésitez pas à me contacter en soumettant une issue sur GitHub à l'adresse: { $link }
solutions = Vous pouvez trouver des solutions à de nombreuses questions fréquemment posées à l'adresse: { $link }

using-old-version = Vous utilisez une version plus ancienne ({ $package_version }) du bot. Téléchargez la dernière version ({ $release_version }) à partir de { $link }.

ffmpeg = FFmpeg est disponible
    .not_installed = FFmpeg n'est pas installé sur ce système.
    .auto_download = Nous pouvons essayer de l'installer automatiquement pour vous. Voulez-vous le faire ? (y/n)
    .manually = Veuillez installer FFmpeg manuellement et réessayer.
    .downloading = Téléchargement de FFmpeg.

assets = Les actifs sont disponibles !
    .empty = La liste des actifs est vide. Veuillez vérifier votre configuration et vous assurer qu'au moins un fichier vidéo ou audio est spécifié.
    .wrong-mime = Le type mime de { $file } est { $actual } mais il est attendu { $expected }.
        Veuillez vous assurer que le fichier est du type correct avant de continuer.
    .downloading = Téléchargement d'un autre actif...

reddit = *
    .credentials = Vos identifiants Reddit sont incorrects ! Veuillez les modifier en conséquence dans le fichier config.toml.
    .login-success = Hourra ! Je me suis connecté avec succès à Reddit et j'ai appliqué le thème souhaité
    .subreddit-checking = Vérification du subreddit { $name }
    .subreddit-finished = Terminé avec le subreddit !
    .post-skipped = Erreur rencontrée lors du traitement du post du subreddit: { $error }.
    Post ignoré.
    .post-inform = La vidéo sera { $name } 👍
    L'URL du fil est { $link } 👍
    Le fil a un ratio de vote positif de { $percent }%

video = *
    .success = Le contenu généré est disponible à l'adresse { $path } 🥰
    .error = Erreur rencontrée lors du traitement de la vidéo { $error }

task = *
    .spawn-failed = Impossible de démarrer le script externe { $script }, en raison de { $error }
    .finished = La tâche ({ $script }) { $code -> 
        [0] -> s'est terminée correctement.
        *[other] -> a échoué avec le code d'erreur { $code }.
    }