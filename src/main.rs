use text_io::read;

mod imgproc;

fn main() {
    println!("Send the word to convert to image to: ");

    let word: String = read!("{}\n");
    imgproc::convert_text_to_image_file(&word);
}
