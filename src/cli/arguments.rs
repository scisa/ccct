use clap::{Arg, ArgMatches, Command};
use std::error::Error;

use crate::cli::arg_constants::*;
use crate::util::error_messages::*;
use crate::util::exit_codes::*;

#[derive(Debug)]
pub struct Args {
    pub is_enc_mode: bool,
    pub is_dec_mode: bool,
    pub is_insert_key_mode: bool,
    pub args_encrypt: ArgsEncrypt,
    pub args_decrypt: ArgsDecrypt,
    pub args_insert_key: ArgsInsertKey,
}

#[derive(Debug)]
pub struct ArgsEncrypt {
    pub enc_file: String,
    pub no_hash: bool,
}

#[derive(Debug)]
pub struct ArgsDecrypt {
    pub dec_file: String,
    pub dec_key: String,
}

#[derive(Debug)]
pub struct ArgsInsertKey {
    pub insert_key_file: String,
    pub insert_key_key: String,
}

impl Args {
    pub fn get_args() -> Self {
        let matches = Self::parse_args();
        let args = match matches {
            Ok(m) => m,
            Err(e) => {
                eprintln!("{}: {}", ERROR_EXTRACTING_ARGS_NOT_POSSIBLE, e);
                std::process::exit(EXIT_EXTRACTING_ARGS_FAILED)
            }
        };

        Self {
            is_enc_mode: Self::extract_is_enc_mode(&args),
            is_dec_mode: Self::extract_is_dec_mode(&args),
            is_insert_key_mode: Self::extract_is_insert_key_mode(&args),
            args_encrypt: Self::extract_args_encrypt(&args),
            args_decrypt: Self::extract_args_decrypt(&args),
            args_insert_key: Self::extract_args_insert_key(&args),
        }
    }

    fn parse_args() -> Result<ArgMatches, Box<dyn Error>> {
        let matches = Command::new(APP_NAME)
            .version(APP_VERSION)
            .author(APP_AUTHOR)
            .about(APP_DESCRIPTION)
            .subcommand(
                Command::new(KEY_ENCRYPT)
                    .about(ABOUT_ENCRYPT)
                    .arg(
                        Arg::new(KEY_ENC_FILE)
                            .help(HELP_ENC_FILE)
                            .value_name(VALUE_ENC_FILE)
                            .required(true)
                            .short('i')
                            .takes_value(true)
                            .long(LONG_ARG_ENC_FILE),
                    )
                    .arg(
                        Arg::new(KEY_NO_HASH)
                            .help(HELP_NO_HASH)
                            .value_name(VALUE_NO_HASH)
                            .required(false)
                            .short('n')
                            .takes_value(false)
                            .long(LONG_NO_HASH),
                    ),
                
            )
            .subcommand(
                Command::new(KEY_DECRYPT)
                    .about(ABOUT_DECRYPT)
                    .arg(
                        Arg::new(KEY_DEC_FILE)
                            .help(HELP_DEC_FILE)
                            .value_name(VALUE_DEC_FILE)
                            .required(true)
                            .short('i')
                            .takes_value(true)
                            .long(LONG_ARG_DEC_FILE),
                    )
                    .arg(
                        Arg::new(KEY_DEC_KEY)
                            .help(HELP_DEC_KEY)
                            .value_name(VALUE_DEC_KEY)
                            .required(true)
                            .short('k')
                            .takes_value(true)
                            .long(LONG_ARG_DEC_KEY),
                    ),
            )
            .subcommand(
                Command::new(KEY_INSERT_KEY)
                    .about(ABOUT_INSERT_KEY)
                    .arg(
                        Arg::new(KEY_INSERT_KEY_FILE)
                            .help(HELP_INSERT_KEY_FILE)
                            .value_name(VALUE_INSERT_KEY_FILE)
                            .required(true)
                            .short('i')
                            .takes_value(true)
                            .long(LONG_ARG_INSERT_KEY_FILE),
                    )
                    .arg(
                        Arg::new(KEY_INSERT_KEY_KEY)
                            .help(HELP_INSERT_KEY_KEY)
                            .value_name(VALUE_INSERT_KEY_KEY)
                            .required(true)
                            .short('k')
                            .takes_value(true)
                            .long(LONG_ARG_INSERT_KEY_KEY),
                    ),
            )
            .get_matches();
        Ok(matches)
    }

    fn extract_is_enc_mode(args: &ArgMatches) -> bool {
        if args.subcommand_name().unwrap().to_string() == KEY_ENCRYPT {
            return true;
        }
        return false;
    }

    fn extract_is_dec_mode(args: &ArgMatches) -> bool {
        if args.subcommand_name().unwrap().to_string() == KEY_DECRYPT {
            return true;
        }
        return false;
    }

    fn extract_is_insert_key_mode(args: &ArgMatches) -> bool {
        if args.subcommand_name().unwrap().to_string() == KEY_INSERT_KEY {
            return true;
        }
        return false;
    }

    fn extract_args_encrypt(args: &ArgMatches) -> ArgsEncrypt {
        let is_enc_mode = Self::extract_is_enc_mode(&args);

        ArgsEncrypt {
            enc_file: Self::extract_enc_file(&args, is_enc_mode),
            no_hash: Self::extract_no_hash(&args, is_enc_mode),
        }
    }

    fn extract_enc_file(args: &ArgMatches, is_enc_mode: bool) -> String {
        let mut enc_file = String::new();
        if is_enc_mode {
            enc_file = match args.subcommand() {
                Some((KEY_ENCRYPT, sub_matches)) => sub_matches
                    .get_one::<String>(KEY_ENC_FILE)
                    .unwrap()
                    .to_string(),
                Some((&_, _)) => {
                    eprintln!("{}", ERROR_EXTRACTING_ENC_FILE_NOT_POSSIBLE);
                    std::process::exit(EXIT_EXTRACTING_ENC_FILE_FAILED);
                }
                None => {
                    eprintln!("{}", ERROR_EXTRACTING_ENC_FILE_NOT_POSSIBLE);
                    std::process::exit(EXIT_EXTRACTING_ENC_FILE_FAILED);
                }
            };
        }

        enc_file
    }

    fn extract_no_hash(args: &ArgMatches, is_enc_mode: bool) -> bool {
        let mut no_hash = false;
        if is_enc_mode {
            no_hash = match args.subcommand() {
                Some((KEY_ENCRYPT, sub_matches)) => sub_matches.contains_id(KEY_NO_HASH),
                Some((&_, _)) => {
                    eprintln!("{}", ERROR_EXTRACTING_ENC_FILE_NOT_POSSIBLE);
                    std::process::exit(EXIT_EXTRACTING_ENC_FILE_FAILED);
                }
                None => {
                    eprintln!("{}", ERROR_EXTRACTING_ENC_FILE_NOT_POSSIBLE);
                    std::process::exit(EXIT_EXTRACTING_ENC_FILE_FAILED);
                }
            };
        }

        no_hash
    }

    fn extract_args_decrypt(args: &ArgMatches) -> ArgsDecrypt {
        let is_dec_mode = Self::extract_is_dec_mode(&args);

        ArgsDecrypt {
            dec_file: Self::extract_dec_file(&args, is_dec_mode),
            dec_key: Self::extract_dec_key(&args, is_dec_mode),
        }
    }

    fn extract_dec_file(args: &ArgMatches, is_dec_mode: bool) -> String {
        let mut dec_file = String::new();
        if is_dec_mode {
            dec_file = match args.subcommand() {
                Some((KEY_DECRYPT, sub_matches)) => sub_matches
                    .get_one::<String>(KEY_DEC_FILE)
                    .unwrap()
                    .to_string(),
                Some((&_, _)) => {
                    eprintln!("{}", ERROR_EXTRACTING_DEC_FILE_NOT_POSSIBLE);
                    std::process::exit(EXIT_EXTRACTING_DEC_FILE_FAILED);
                }
                None => {
                    eprintln!("{}", ERROR_EXTRACTING_DEC_FILE_NOT_POSSIBLE);
                    std::process::exit(EXIT_EXTRACTING_DEC_FILE_FAILED);
                }
            };
        }

        dec_file
    }

    fn extract_dec_key(args: &ArgMatches, is_dec_mode: bool) -> String {
        let mut dec_key = String::new();
        if is_dec_mode {
            dec_key = match args.subcommand() {
                Some((KEY_DECRYPT, sub_matches)) => sub_matches
                    .get_one::<String>(KEY_DEC_KEY)
                    .unwrap()
                    .to_string(),
                Some((&_, _)) => {
                    eprintln!("{}", ERROR_EXTRACTING_DEC_KEY_NOT_POSSIBLE);
                    std::process::exit(EXIT_EXTRACTING_DEC_KEY_FAILED);
                }
                None => {
                    eprintln!("{}", ERROR_EXTRACTING_DEC_KEY_NOT_POSSIBLE);
                    std::process::exit(EXIT_EXTRACTING_DEC_KEY_FAILED);
                }
            };
        }

        dec_key
    }

    fn extract_args_insert_key(args: &ArgMatches) -> ArgsInsertKey {
        let is_insert_key_mode = Self::extract_is_insert_key_mode(&args);

        ArgsInsertKey {
            insert_key_file: Self::extract_insert_key_file(&args, is_insert_key_mode),
            insert_key_key: Self::extract_insert_key_key(&args, is_insert_key_mode),
        }
    }

    fn extract_insert_key_file(args: &ArgMatches, is_insert_key_mode: bool) -> String {
        let mut insert_key_file = String::new();
        if is_insert_key_mode {
            insert_key_file = match args.subcommand() {
                Some((KEY_INSERT_KEY, sub_matches)) => sub_matches
                    .get_one::<String>(KEY_INSERT_KEY_FILE)
                    .unwrap()
                    .to_string(),
                Some((&_, _)) => {
                    eprintln!("{}", ERROR_EXTRACTING_INSERT_KEY_FILE_NOT_POSSIBLE);
                    std::process::exit(EXIT_EXTRACTING_INSERT_KEY_FILE_FAILED);
                }
                None => {
                    eprintln!("{}", ERROR_EXTRACTING_INSERT_KEY_FILE_NOT_POSSIBLE);
                    std::process::exit(EXIT_EXTRACTING_INSERT_KEY_FILE_FAILED);
                }
            };
        }

        insert_key_file
    }

    fn extract_insert_key_key(args: &ArgMatches, is_insert_key_mode: bool) -> String {
        let mut insert_key_key = String::new();
        if is_insert_key_mode {
            insert_key_key = match args.subcommand() {
                Some((KEY_INSERT_KEY, sub_matches)) => sub_matches
                    .get_one::<String>(KEY_INSERT_KEY_KEY)
                    .unwrap()
                    .to_string(),
                Some((&_, _)) => {
                    eprintln!("{}", ERROR_EXTRACTING_INSERT_KEY_KEY_NOT_POSSIBLE);
                    std::process::exit(EXIT_EXTRACTING_INSERT_KEY_KEY_FAILED);
                }
                None => {
                    eprintln!("{}", ERROR_EXTRACTING_INSERT_KEY_KEY_NOT_POSSIBLE);
                    std::process::exit(EXIT_EXTRACTING_INSERT_KEY_KEY_FAILED);
                }
            };
        }

        insert_key_key
    }
}
