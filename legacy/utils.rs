pub fn create_callback() -> Callback {
    Callback {
        skipping_post : |error| eprintln!("{error}"),
        dimesions_out_of_bounds : |cd,vd|{
            eprintln!("Dimesions set {}x{} , are bigger then video dimensions {}x{}.\nHence we are not cropping the video the the specificed dimesions",cd.width,cd.height,vd.width,vd.height);
        }
    }
}