use crate::FileType::Binary;
use std::{io, ops::Index};


use anyhow::Result;

pub enum FileType{
    Text,Binary
}

enum Node {
    File(File),
    Dir(Dir)
}

struct File{
    name : String,
    content: Vec<u8>,
    creation_time:u64,
    type_:FileType
}

struct Dir{
    name  : String,
    creation_time : u64,
    children : Vec<Node>
}

struct FileSystem{
    root : Dir
}

impl File{
    fn new(ftype: FileType) -> File {
        return File{name : "".to_string(),content : Vec::new(),creation_time : 0,type_: ftype}
    }
}

impl Dir{
    fn new(path : &str) -> Dir {
        return Dir{name:path.to_string(),creation_time : 0, children : Vec::new()}
    }
}

impl FileSystem{
    fn new() -> Self {
        let root_dir = Dir::new("D:/");
        return FileSystem { root : root_dir} 
    }

    fn from_dir(path : &str) -> Self {
        let root_dir = Dir::new(path);
        return  FileSystem {root : root_dir};
    }

    fn mk_dir(self,path : &str) -> Result<&Dir>{
        let mut directories : Vec<&str> = path.split('/').collect();
        directories = directories[1..directories.len()-1].to_vec();
        let dir_name = directories[directories.len()-1];
        //ad es D:/a/b/c viene divisa in 'D:' ,'a' ,'b' e 'c'
        //usa una funzione di ricerca che partendo da FileSystem -> root vada a cercare lungo i figli di D: la cartella 'a', lungo i figli di 'a' cerca 'b' e così via
        
        //trova il direttorio padre
        let parent_dir:Option<&mut Dir> = self.search(directories);

        match parent_dir {
            None  => return Err(anyhow::Error::new(io::Error::new(io::ErrorKind::NotFound, "File not found"))),
            Some(parent_dir) => {
                                        let dir = Dir::new(dir_name);

                                        parent_dir.children.push(Node::Dir(dir));
                                        return Ok(&parent_dir.children[parent_dir.children.len()]);
            }
        };
        
        
    }

    fn rm_dir(self,path:&str) -> anyhow::Result<&Dir>{
        let mut directories : Vec<&str> = path.split('/').collect();
        directories = directories[1..directories.len()-1].to_vec();
        let dir_name = directories[directories.len()-1];

        //trova il direttorio padre
        let parent_dir:&mut Dir = self.search(directories).expect("Impossibile trovare il direttorio padre in rm_dir");
        let mut index = 0;
        for child in parent_dir.children {

            match child{
                Node::Dir(dir) => {
                    if dir.name == dir_name && dir.children.is_empty(){
                        parent_dir.children.remove(index);
                        return Ok(parent_dir);
                    }
                }
                Node::File(_)=> continue 
            }
            index+=1;
        }
        
        return Err(anyhow::Error::new(io::Error::new(io::ErrorKind::NotFound, "Directory figlia non trovata in rm_dir")));

    }

}

    


fn main() {
    println!("Hello, world!");
}
