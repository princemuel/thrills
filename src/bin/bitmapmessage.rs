//! Bitmap Message, by Prince Muel info@princemuel.dev
//! Displays a text message according to the provided bitmap image.
//! This code is available at https://github.com/princemuel/thrills
//! Tags: tiny, beginner, artistic
use std::io::{self, Write as _};

use unicode_segmentation::UnicodeSegmentation;

const BITMAP: &str = r#"
....................................................................
   **************   *  *** **  *      ******************************
  ********************* ** ** *  * ****************************** *
 **      *****************       ******************************
          *************          **  * **** ** ************** *
           *********            *******   **************** * *
            ********           ***************************  *
   *        * **** ***         *************** ******  ** *
               ****  *         ***************   *** ***  *
                 ******         *************    **   **  *
                 ********        *************    *  ** ***
                   ********         ********          * *** ****
                   *********         ******  *        **** ** * **
                   *********         ****** * *           *** *   *
                     ******          ***** **             *****   *
                     *****            **** *            ********
                    *****             ****              *********
                    ****              **                 *******   *
                    ***                                       *    *
                    **     *                    *
...................................................................."#;

fn main() -> io::Result<()> {
    println!("Bitmap Message, by Al Sweigart al@inventwithpython.com");
    println!("Enter the message to display with the bitmap.");

    let message = read_line()?;

    let message: Vec<_> = UnicodeSegmentation::graphemes(message.trim(), true).collect();
    // let mut symbols = graphemes.iter().cycle();
    let n = message.len();

    for line in BITMAP.lines() {
        // TODO: there should be a way to

        for (i, bit) in line.char_indices() {
            if bit == ' ' {
                print!(" ");
            } else {
                print!("{}", message[i % n]);
                // avoids bounds checking
                // print!("{}", symbols.next().unwrap());
            }
        }

        println!();
    }

    Ok(())
}

fn read_line() -> io::Result<String> {
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        if !buffer.trim().is_empty() {
            return Ok(buffer);
        }
    }
}
