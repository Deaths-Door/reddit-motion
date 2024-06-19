
from reddit_motion import RedditMotionCommandLineParser;
from youtube_uploader import VALID_PRIVACY_STATUSES, get_authenticated_service, initialize_upload as upload_youtube_video
from apiclient.errors import HttpError
from pathlib import Path
from tiktok_uploader.upload import upload_videos as upload_tiktok_videos

def main(all_file_paths) :
    file_metadata = [(file_path,Path(file_path).stem) for file_path in all_file_paths]

    # Upload Videos to Youtube first
    youtube = get_authenticated_service(None)

    failed_uploads = []
    
    for path , name in file_metadata :
        failed_upload = upload_youtube_video(youtube, {
            "title" : name ,
            "privacyStatus" : VALID_PRIVACY_STATUSES[0] ,
            "category" : 22,
            "file" : path
        })

        if failed_upload is not None : 
            failed_uploads.append(failed_upload)

    # Now upload tiktok videos -- Look at https://pypi.org/project/tiktok-uploader/ for requirements (cookies.txt)
    _failed_uploads = upload_tiktok_videos([{ "path" : path ,"description" : name} for path , name in file_metadata],cookies = "tiktok_cookies.txt")
    failed_uploads.append(_failed_uploads)

    save_failed_uploads_to_file(failed_uploads)

def run_failed_uploads() :
    with open("upload_errors.txt", "a") as file:
        all_file_paths = [line for line in file]
    main(all_file_paths)
    
def save_failed_uploads_to_file(failed_uploads) :
    with open("upload_errors.txt", "a") as error_file:
        for path in failed_uploads:
            error_file.write(f"{path}\n")
    pass

if __name__ == "__main__" :
    run_failed_uploads()

    arguments = RedditMotionCommandLineParser()
    arguments.parse()

    main(arguments.all_file_paths())