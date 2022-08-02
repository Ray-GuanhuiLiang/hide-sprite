extern crate core;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::process::exit;
use clap::Parser;

#[derive(Parser)]
struct Options {
    /// hide mode, hide the input file
    #[clap(long)]
    hide: bool,

    /// file name to be handled. hide or show
    #[clap(value_parser)]
    filename: String,
}

const MAGIC_CODE: [u8; 5] = [113, 234, 6, 57, 34];
const BUF_SIZE: usize = 32 * 1024;

fn main() {
    let opt:Options = Options::parse();
    // println!("hide={:?}", opt.hide);
    // println!("filename={:?}", opt.filename);
    match process(&opt) {
        Ok(()) => {
            println!("process success")
        },
        Err(e) => {
            eprintln!("error process: {}", e);
            exit(100);
        },
    }
}

fn process(opt: &Options) -> Result<(), std::io::Error> {
    let src_path = Path::new(&opt.filename);
    if !src_path.exists() {
        eprintln!("[error] file is not exist: {}", opt.filename);
        exit(1);
    }
    let hs_fn = opt.filename.clone() + &".hs";
    let hs_path = Path::new(&hs_fn);
    return if opt.hide {
        hide(src_path, hs_path)
    } else {
        show(src_path, hs_path)
    }
}

fn show(src_path: &Path, hs_path: &Path) -> Result<(), std::io::Error> {
    if ! hs_path.exists() {
        eprintln!("[error] hs file is not existing: {:?}", hs_path.as_os_str());
        exit(2);
    }
    let mut buf : [u8; BUF_SIZE] = [0; BUF_SIZE];
    let mut head : [u8; 5] = [0; 5];
    let mut fp = OpenOptions::new()
        .read(true)
        .write(true)
        .open(src_path)?;
    fp.read(&mut head)?;
    if head != MAGIC_CODE {
        exit(10);
    }
    let mut fp2 = File::open(hs_path)?;
    fp2.read(&mut buf)?;
    fp.seek(SeekFrom::Start(0))?;
    fp.write_all(&buf)?;

    drop(fp2);
    fs::remove_file(hs_path)?;
    Ok(())
}

fn hide(src_path: &Path, hs_path: &Path) -> Result<(), std::io::Error> {
    if hs_path.exists() {
        eprintln!("[error] hs file is existing: {:?}", hs_path.as_os_str());
        exit(3);
    }
    let mut buf : [u8; BUF_SIZE] = [0; BUF_SIZE];
    let mut fp = OpenOptions::new()
        .read(true)
        .write(true)
        .open(src_path)?;
    // 读取源文件
    let read = fp.read(&mut buf)?;
    if read < buf.len() {
        eprintln!("[error] not enough file length: {}", read);
        exit(20);
    }
    // 写入.hs文件
    let mut fp2 = File::create(hs_path)?;
    fp2.write_all(&buf)?;
    // 更新源文件的头
    buf.fill(0);
    buf[..MAGIC_CODE.len()].clone_from_slice(&MAGIC_CODE);
    fp.seek(SeekFrom::Start(0))?;
    fp.write_all(&MAGIC_CODE)?;

    Ok(())
}
