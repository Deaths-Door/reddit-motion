thanks = Bu aracı kullandığınız için teşekkür ederiz! Bu projeye GitHub'da katkıda bulunmaktan çekinmeyin!

questions = Herhangi bir sorunuz varsa, lütfen GitHub Issue aracılığıyla benimle iletişime geçin: { $link }

solutions = Birçok SSS'nin çözümlerini { $link } adresinde bulabilirsiniz.

using-old-version = Botun eski bir sürümünü ( { $package_version } ) kullanıyorsunuz. En son sürüm ({ $release_version } ) 'ü { $link } adresinden indirin.

ffmpeg = FFmpeg kullanılabilir
    .not_installed = Bu sistemde FFmpeg kurulu değil.
    .auto_download = Bunu sizin için otomatik olarak yüklemeyi deneyebiliriz. Bunu yapmak ister misiniz? (e/h)
    .manually = Lütfen FFmpeg'yi manuel olarak kurun ve tekrar deneyin.
    .downloading = FFmpeg indiriliyor.

assets = Varlıkları kullanılabilir!
    .empty = Varlıklar listesi boş. Lütfen yapılandırmanızı kontrol edin ve en az bir video veya ses dosyası belirtildiğinden emin olun.
    .wrong-mime = { $file } dosyasının mime türü { $actual } ancak beklenen { $expected } türüdür.
        Devam etmeden önce dosyanın doğru türde olduğundan emin olun.
    .downloading = Başka bir varlık indiriliyor...

reddit = *
    .credentials = Reddit kimlik bilgileriniz yanlış! Lütfen bunları config.toml dosyasına göre değiştirin.
    .login-success = Yaaay! Reddit'e başarıyla giriş yaptım ve istenen temayı uyguladım
    .subreddit-checking = { $name } subreddit'ini kontrol ediyor
    .subreddit-finished = Subreddit ile bitti!
    .post-skipped = subreddit gönderisini işlerken hata oluştu: { $error }.
    Post atlanıyor.
    .post-inform = Video { $name } olacak 👍
    Konu URL'si { $link } 👍
    Konu { $percent }% oy oranına sahip

video = *
    .success = Oluşturulan içerikler { $path } adresinde mevcuttur 🥰
    .error = Videoyu işlerken hata oluştu: { $error }

task = *
    .spawn-failed = Dışarıdan bir komut dosyası { $script } başlatılamadı, çünkü { $error }.
    .finished = Görev ({ $script }) { $code ->
        [0] -> Başarıyla tamamlandı.
        *[other] -> Hata kodu { $code } ile başarısız oldu.
    }