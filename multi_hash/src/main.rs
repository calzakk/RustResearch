use std::env;
use std::fs::File;
use std::hash::Hasher;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("One pathname expected");
    }
    let pathname = &args[1];

    let mut sha1hasher = rs_sha1::Sha1Hasher::default();
    let mut sha224hasher = rs_sha224::Sha224Hasher::default();
    let mut sha256hasher = rs_sha256::Sha256Hasher::default();
    let mut sha384hasher = rs_sha384::Sha384Hasher::default();
    let mut sha512hasher = rs_sha512::Sha512Hasher::default();
    let mut sha512_224hasher = rs_sha512_224::Sha512_224Hasher::default();
    let mut sha512_256hasher = rs_sha512_256::Sha512_256Hasher::default();

    let file_result = File::open(pathname);
    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("{:?}", error);
        }
    };

    let mut data = vec![0; 1_000_000];
    loop {
        let result = file.read(data.as_mut_slice());
        match result {
            Ok(size) => {
                if size == 0 {
                    break;
                }
                let slice = &data[0..size];
                sha1hasher.write(slice);
                sha256hasher.write(slice);
                sha224hasher.write(slice);
                sha384hasher.write(slice);
                sha512hasher.write(slice);
                sha512_224hasher.write(slice);
                sha512_256hasher.write(slice);
            },
            Err(err) => {
                println!("{}", err);
                break;
            }
        }
    }

    sha1hasher.finish();
    sha224hasher.finish();
    sha256hasher.finish();
    sha384hasher.finish();
    sha512hasher.finish();
    sha512_224hasher.finish();
    sha512_256hasher.finish();

    let bytes1 = rs_sha1::HasherContext::finish(&mut sha1hasher);
    let bytes224 = rs_sha224::HasherContext::finish(&mut sha224hasher);
    let bytes256 = rs_sha256::HasherContext::finish(&mut sha256hasher);
    let bytes384 = rs_sha384::HasherContext::finish(&mut sha384hasher);
    let bytes512 = rs_sha512::HasherContext::finish(&mut sha512hasher);
    let bytes512_224 = rs_sha512_224::HasherContext::finish(&mut sha512_224hasher);
    let bytes512_256 = rs_sha512_256::HasherContext::finish(&mut sha512_256hasher);

    println!("SHA-1: {bytes1:02x}");
    println!("SHA-224: {bytes224:02x}");
    println!("SHA-256: {bytes256:02x}");
    println!("SHA-384: {bytes384:02x}");
    println!("SHA-512: {bytes512:02x}");
    println!("SHA-512/224: {bytes512_224:02x}");
    println!("SHA-512/256: {bytes512_256:02x}");
}
