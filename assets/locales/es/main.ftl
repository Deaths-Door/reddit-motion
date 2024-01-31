thanks = ¡Gracias por usar esta herramienta! ¡No dude en contribuir a este proyecto en GitHub!

questions = Si tiene alguna pregunta, no dude en ponerse en contacto conmigo enviando una incidencia en GitHub a la siguiente dirección: { $link }

solutions = Puede encontrar soluciones a muchas preguntas frecuentes en { $link }

using-old-version = Está utilizando una versión anterior del bot ({ $package_version }). Descargue la última versión ({ $release_version }) desde { $link }.

ffmpeg = FFmpeg está disponible
    .not_installed = FFmpeg no está instalado en este sistema.
    .auto_download = Podemos intentar instalarlo automáticamente para usted. ¿Lo desea? (s/n)
    .manually = Instale FFmpeg manualmente y vuelva a intentarlo.
    .downloading = Descarga de FFmpeg.

assets = ¡Los activos están disponibles!
    .empty = La lista de activos está vacía. Revise su configuración y asegúrese de que se haya especificado al menos un archivo de video o audio.
    .wrong-mime = El tipo mime del archivo { $file } es { $actual } pero se espera { $expected }.
        Asegúrese de que el archivo esté del tipo correcto antes de continuar.
    .downloading = Descargando otro activo...

reddit = *
    .credentials = ¡Sus credenciales de Reddit son incorrectas! ¡Cáncelalas y vuelve a intentarlo!
    .login-success = ¡Hurra! ¡He iniciado sesión con éxito en Reddit y apliqué el tema deseado!
    .subreddit-checking = Verificando el subreddit { $name }
    .subreddit-finished = ¡Listo con el subreddit!
    .post-skipped = Se encontró un error al procesar la publicación del subreddit: { $error }.
    Saltar publicación.
    .post-inform = ¡El video será { $name } 👍!
    ¡La URL del hilo es { $link } 👍!
    ¡El hilo tiene un índice de votación positivo del { $percent }%!

video = *
    .success = ¡El contenido generado está disponible en { $path } 🥰!
    .error = Se encontró un error al procesar el video { $error }

task = *
    .spawn-failed = No se pudo iniciar el script externo { $script }, debido a { $error }
    .finished = La tarea ({ $script }) { $code -> 
        [0] -> se completó correctamente.
        *[other] -> falló con el código de error { $code }.
    }