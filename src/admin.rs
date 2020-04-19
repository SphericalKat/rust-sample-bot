use teloxide::prelude::*;
use teloxide::types::InputFile;

pub async fn promote(cx: &DispatcherHandlerCx<Message>, _args: &[String]) -> ResponseResult<()> {
    let message_result = cx.answer_photo(InputFile::url("https://image.shutterstock.com/image-vector/sample-stamp-square-grunge-sign-260nw-1474408826.jpg"))
        .caption("Sample pic")
        .reply_to_message_id(cx.update.id).send().await;

    match message_result {
        Ok(msg) => {
            let photo = msg.photo().expect("Expected a photo");
            println!("File ID: {}", photo[0].file_id);
        }

        Err(err) => println!("{}", err),
    }

    Ok(())
}
