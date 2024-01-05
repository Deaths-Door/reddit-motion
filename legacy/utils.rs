pub fn create_callback() -> Callback {
    Callback {
        info : |submission| {
            println!("Video will be {} 👍",submission.title);
            println!("Thread url is https://reddit.com{} 👍",submission.permalink);
            println!("Thread has a upvote ratio of {}%",submission.upvote_ratio);
        },
        skipping_post : |error| eprintln!("{error}"),
        dimesions_out_of_bounds : |cd,vd|{
            eprintln!("Dimesions set {}x{} , are bigger then video dimensions {}x{}.\nHence we are not cropping the video the the specificed dimesions",cd.width,cd.height,vd.width,vd.height);
        }
    }
}