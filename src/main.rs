pub mod cli;
pub mod input;
pub mod crypt;
pub mod output;
pub mod util;

use cli::arguments::Args;
use crypt::encrypt::Encrypted;
use crypt::decrypt::Decrypted;
use input::read_data;
use output::write_enc_file;
use output::print_to_cli;


fn main() {
    // fetch args
    let args = Args::get_args();

    // select mode
    if args.is_enc_mode {
        use_encryption_mode(&args);
    } else {
        use_decryption_mode(&args);
    }
}

fn use_encryption_mode(args: &Args) {
    // read in file
    let data = read_data::get_data(&args.args_encrypt.enc_file);
    
    // encrypt read in data
    let encrypted_data = Encrypted::encrypt(&data);

    // write to encrypted file
    write_enc_file::write_encrypted_file(&args.args_encrypt.enc_file, &encrypted_data.enc_data);

    // write key to cli
    print_to_cli::print_hex_key(&encrypted_data.key_hex_string);
}

fn use_decryption_mode(args: &Args) {
    // let mut enc_data: Vec<u8> = vec![39, 23, 44, 193, 185, 87, 223, 74, 181, 77, 131, 137, 168, 122, 243, 119, 233, 72, 255, 135, 175, 88, 0, 219, 127, 179, 87, 235, 62, 115, 208, 229, 143, 48, 202, 71, 37, 188, 129, 14, 96, 94, 202, 222, 236, 38, 114, 93, 3, 139, 26, 119, 61, 193, 2, 50, 252, 144, 191, 30, 159, 75, 151, 207, 173, 103, 102, 98, 166, 63, 40, 161, 59, 133, 103, 207, 175, 128];
    let mut enc_bytes = input::read_data::read_enc_data(&args.args_decrypt.dec_file);

    // decrypt data
    let decrypted_data = Decrypted::decrypt(&args.args_decrypt.dec_key, &mut enc_bytes);

    output::print_to_cli::print_dec_plaintext(&decrypted_data.plaintext);
}