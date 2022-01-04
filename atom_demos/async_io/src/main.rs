use std::{error::Error, fs::Metadata};
use std::path::PathBuf;
use humansize::{file_size_opts, FileSize};
//use std::fs;
use tokio::fs;
use futures::{TryFutureExt, TryStreamExt};
use futures::stream::FuturesUnordered;
use futures::future::ok;

//同步
// fn cal_space_size(path: PathBuf) -> std::io::Result<u64> {
//     let mut paths = vec![path];
//     let mut res_size = 0;
//     while let Some(path) = paths.pop() {
//         let meta = fs::symlink_metadata(&path)?;
//         let file_type = meta.file_type();
//         if file_type.is_dir() {
//             let entries = fs::read_dir(path)?;
//             for entry in entries {
//                 paths.push(entry?.path());
//             }
//         }
//         if file_type.is_file() {
//             res_size += meta.len();
//         }
//     }
//     Ok(res_size)
// }

//异步错误写法
// async fn cal_space_size(path: PathBuf) -> std::io::Result<u64> {
//     let mut paths = vec![path];
//     let mut res_size = 0;
//     while let Some(path) = paths.pop() {
//         let meta = fs::symlink_metadata(&path).await?;
//         let file_type = meta.file_type();
//         if file_type.is_dir() {
//             let mut entries = fs::read_dir(path).await?;
//             while let Some(entry) = entries.next_entry().await? {
//                 paths.push(entry.path())
//             }
//         }
//         if file_type.is_file() {
//             res_size += meta.len();
//         }
//     }
//     Ok(res_size)
// }

async fn cal_space_size(path: PathBuf) -> std::io::Result<u64> {
    let mut meta_queue = FuturesUnordered::new();
    meta_queue.push(metadata_for_path(path));
    let mut res_size = 0;
    while let Some((path,meta)) = meta_queue.try_next().await? {
        let file_type = meta.file_type();
        if file_type.is_dir() {
            let mut entries = fs::read_dir(path).await?;
            while let Some(entry) = entries.next_entry().await? {
                meta_queue.push(metadata_for_path(entry.path()));
            }
        }
        if file_type.is_file() {
            res_size += meta.len();
        }
    }
    Ok(res_size)
}
//获取path的metadata
async fn metadata_for_path(path: PathBuf) -> std::io::Result<(PathBuf,Metadata)> {
    let meta = fs::symlink_metadata(&path).await?;
    Ok((path,meta))
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("输入目录名")
    }
    //同步写法
    // println!("{}",args[1]);
    // let space_size = cal_space_size(PathBuf::from(args[1].to_string()))?;
    // println!("space size: {} \t {}",args[1],space_size.file_size(file_size_opts::CONVENTIONAL)?);
    // Ok(())
    let space_size = cal_space_size(PathBuf::from(args[1].to_string())).await?;
    let usage = space_size.file_size(file_size_opts::CONVENTIONAL)?;
    println!("space size:{} \t {}",args[1],usage);
    Ok(())
}
