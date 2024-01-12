thanks = Grazie per aver utilizzato questo strumento! Non esitare a contribuire a questo progetto su GitHub!

questions = Se hai domande, non esitare a contattarmi tramite un ticket su GitHub all'indirizzo: { $link }

solutions = Puoi trovare soluzioni a molte FAQ comuni su { $link }

using-old-version = "Stai utilizzando una versione precedente del bot ({ $package_version }). Scarica l'ultima versione ({ $release_version }) da { $link }.

ffmpeg = FFmpeg è disponibile
    .not_installed = FFmpeg non è installato su questo sistema.
    .auto_download = Possiamo provare a installarlo automaticamente per te. Vuoi farlo? (s/n)
    .manually = Installa FFmpeg manualmente e riprova.
    .downloading = Download di FFmpeg.

assets = Gli asset sono disponibili!
    .empty = La lista degli asset è vuota. Controlla la tua configurazione e assicurati che sia specificato almeno un file video o audio.
    .wrong-mime = Il tipo mime del file { $file } è { $actual } ma è expected { $expected }.
        Assicurati che il file sia del tipo corretto prima di continuare.
    .downloading = Downloading di un altro asset...

reddit = *
    .credentials = I tuoi dati di accesso a Reddit sono errati! Modificali di conseguenza nel file config.toml
    .login-success = Evviva! Ho effettuato l'accesso con successo a Reddit e ho applicato il tema desiderato.
    .subreddit-checking = Controllo del subreddit { $name }
    .subreddit-finished = Finito con il subreddit!
    .post-skipped = Errore riscontrato durante l'elaborazione del post del subreddit: { $error } \n. Salta post.
    .post-inform = "Il video sarà { $name } 👍\n L'URL del thread è { $link } 👍\nIl thread ha un rapporto di voti positivi del { $percent }%

video = *
    .success = Il contenuto generato è disponibile all'indirizzo { $path } 🥰
    .error = Errore riscontrato durante l'elaborazione del video { $error }