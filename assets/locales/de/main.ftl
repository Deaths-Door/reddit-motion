thanks = Vielen Dank, dass Sie dieses Tool verwenden! Fühlen Sie sich frei, zu diesem Projekt auf GitHub beizutragen!
questions = Wenn Sie Fragen haben, können Sie mich gerne per GitHub Issue kontaktieren: { $link }
solutions = Lösungen für viele häufig gestellte Fragen finden Sie unter { $link }

using-old-version = Sie verwenden eine ältere Version ({ $package_version }) des Bots. Laden Sie die neueste Version ({ $release_version }) von { $link } herunter.

ffmpeg = FFmpeg ist verfügbar
    .not_installed = FFmpeg ist auf diesem System nicht installiert.
    .auto_download = Wir können versuchen, es automatisch für Sie zu installieren. Möchten Sie das tun? (y/n)
    .manually = Bitte installieren Sie FFmpeg manuell und versuchen Sie es erneut.
    .downloading = FFmpeg wird heruntergeladen.

assets = Assets sind verfügbar!
    .empty = Die Assets-Liste ist leer. Bitte überprüfen Sie Ihre Konfiguration und stellen Sie sicher, dass mindestens eine Video- oder Audiodatei angegeben ist.
    .wrong-mime = Der MIME-Typ von { $file } ist { $actual } aber es wird erwartet { $expected }.
        Bitte stellen Sie sicher, dass die Datei vom richtigen Typ ist, bevor Sie fortfahren.
    .downloading = Weitere Assets werden heruntergeladen...

reddit = *
    .credentials = Ihre Reddit-Anmeldedaten sind ungültig! Bitte ändern Sie sie entsprechend in der config.toml-Datei.
    .login-success = Hurra! Wurde erfolgreich bei Reddit angemeldet und das gewünschte Thema angewendet
    .subreddit-checking = Überprüfe Subreddit { $name }
    .subreddit-finished = Mit dem Subreddit fertig!
    .post-skipped = Fehler beim Verarbeiten des Subreddit-Beitrags: { $error }.
    Beitrag wird übersprungen.
    .post-inform = Video wird { $name } 👍
    Thread-URL ist { $link } 👍
    Thread hat eine Upvote-Ratio von { $percent }%

video = *
    .success = Erstellte Inhalte sind unter { $path } verfügbar 🥰
    .error = Fehler beim Verarbeiten des Videos { $error }

task = *
    .spawn-failed = Starten des externen Skripts { $script } nicht möglich, aufgrund von { $error }
    .finished =  Die Aufgabe ({ $script }) { $code ->
        [0] -> wurde erfolgreich abgeschlossen.
        *[other] -> ist mit dem Fehlercode { $code } fehlgeschlagen.
    }