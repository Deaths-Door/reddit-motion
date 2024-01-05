thanks = Thanks for using this tool! Feel free to contribute to this project on GitHub!
questions = If you have any questions, feel free to contact me by submitting a GitHub issue at { $link }
solutions = You can find solutions to many common FAQs at { $link }

using-old-version = "You are using an older version ({ $package_version }) of the bot. Download the newest version ({ $release_version }) from { $link }

ffmpeg = FFmpeg is availiable
    .not_installed = FFmpeg is not installed on this system.
    .auto_download = We can try to automatically install it for you. Would you like to do that? (y/n)
    .manually = Please install FFmpeg manually and try again.
    .downloading = Downloading FFmpeg

assets = Assets are availiable!
    .empty = The Assets list is empty. Please check your configuration and ensure that there are at least one video or audio file specified.
    .wrong-mime = The mime type of { $file } is { $actual } but it is expected to be { $expected }.
        Please ensure that the file is of the correct type before continuing.
    .downloading = Downloaded another asset...

reddit = PUTINMESSAGE
    .credentials = Your reddit credentials are incorrect! Please modify them accordingly in the config.toml file.
    .login-success = Hooray! Successfully have logged into Reddit and applied the desired theme
    .subreddit-checking = Checking { $name } subreddit
    .subreddit-finished = Finished with subreddit!
    .post-skipped = Error encountered while processing subreddit post: { $error } \n. Skipping post.
    .post-inform = "Video will be { $name } 👍\n Thread url is { $link } 👍\nThread has a upvote ratio of { $percent }%