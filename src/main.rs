pub mod calc_entropy;
pub mod cli;
pub mod crypt;
pub mod input;
pub mod insert;
pub mod output;
pub mod util;

use calc_entropy::entropy;
use cli::arguments::Args;
use crypt::decrypt::Decrypted;
use crypt::encrypt::Encrypted;
use input::read_data;
use output::print_to_cli;
use output::write_enc_file;

fn main() {
    // fetch args
    let args = Args::get_args();

    // select mode
    if args.is_enc_mode {
        use_encryption_mode(&args);
    } else if args.is_dec_mode {
        use_decryption_mode(&args);
    } else if args.is_insert_key_mode {
        use_insert_key_mode(&args);
    }
}

fn use_encryption_mode(args: &Args) {
    // read in file
    let data = read_data::get_data(&args.args_encrypt.enc_file);

    // encrypt read in data
    let encrypted_data = Encrypted::encrypt(&data, args.args_encrypt.no_hash);

    // write to encrypted file
    write_enc_file::write_encrypted_file(&args.args_encrypt.enc_file, &encrypted_data.enc_data);

    // write key to cli
    print_to_cli::print_hex_key(&encrypted_data.key_hex_string);
}

fn use_decryption_mode(args: &Args) {
    // read encrypted data
    let mut enc_bytes = input::read_data::read_byte_data(&args.args_decrypt.dec_file);

    // decrypt data
    let decrypted_data = Decrypted::decrypt(&args.args_decrypt.dec_key, &mut enc_bytes);

    // print plaintext to cli
    output::print_to_cli::print_dec_plaintext(&decrypted_data.plaintext);
}

fn use_insert_key_mode(args: &Args) {
    // read bytes of file
    let mut bytes = input::read_data::read_byte_data(&args.args_insert_key.insert_key_file);

    // insert key to bytes
    insert::insert_key::insert(&args.args_insert_key.insert_key_key, &mut bytes);

    // write back to file
    output::write_insert_key_file::write_insert_key_file(
        &args.args_insert_key.insert_key_file,
        &bytes,
    );

    // display success
    let entropy = entropy::entropy(&args.args_insert_key.insert_key_key);
    output::print_to_cli::print_entropy_of_key(entropy);
}
