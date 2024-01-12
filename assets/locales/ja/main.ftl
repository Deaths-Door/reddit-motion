thanks = このツールをご利用いただきありがとうございます！GitHubでこのプロジェクトに貢献していただくこともできます！

questions = ご質問がございましたら、GitHubのIssueを通じてお問い合わせください: { $link }

solutions = 多くのよくある質問の解決策については、{ $link }をご覧ください。

using-old-version = "古いバージョンのボット ( { $package_version } ) を使用しています。最新バージョン ( { $release_version } ) を { $link } からダウンロードしてください。

ffmpeg = FFmpegが利用可能です
    .not_installed = このシステムにFFmpegがインストールされていません。
    .auto_download = 自動的にインストールを試みることができます。よろしいですか？ (y/n)
    .manually = FFmpegを手動でインストールしてからもう一度やり直してください。
    .downloading = FFmpegをダウンロードしています。

assets = 資産が利用可能です！
    .empty = 資産リストが空です。設定を確認し、少なくとも1つの動画または音声ファイルが指定されていることを確認してください。
    .wrong-mime = { $file }ファイルのMIMEタイプは{ $actual }ですが、{ $expected }が期待されています。
        続行する前に、ファイルが正しいタイプであることを確認してください。
    .downloading = 別の資産をダウンロードしています...

reddit = *
    .credentials = Redditの資格情報が無効です！config.tomlファイルに従って変更してください。
    .login-success = やった！ Redditに正常にログインし、希望のテーマを適用しました。
    .subreddit-checking = { $name }サブredditをチェックしています
    .subreddit-finished = サブreddit終了！
    .post-skipped = サブreddit投稿の処理中にエラーが発生しました：{ $error } \n投稿をスキップします。
    .post-inform = "ビデオは{ $name }になります👍\nスレッドURLは{ $link }👍\nスレッドの投票率は{ $percent }%です。"

video = *
    .success = 生成されたコンテンツは{ $path }にあります🥰！
    .error = ビデオの処理中にエラーが発生しました：{ $error }