use crate::FileType::Binary;
use std::{io, ops::Index, time};


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
        return File{name : "".to_string(),content : Vec::new(),creation_time : time::Instant::now().elapsed().as_secs() as u64,type_: ftype}
    }
}

impl Dir{
    fn new(path : &str) -> Dir {
        return Dir{name:path.to_string(),creation_time : time::Instant::now().elapsed().as_secs() as u64, children : Vec::new()}
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

    fn search_parent(self,path : Vec<&str>) -> Option<&Dir>{
        let mut found=0;
        let mut curr_dir = &(self.root);
        let mut index = 0;

        //cerchiamo lungo tutte le directory del path intero
        let p_len = path.len();
        for dir in path {
            found = 0;
            //se siamo arrivati alla fine del path ritorniamo la directory corrente
            if index == (p_len - 1) {
                let result = &(*curr_dir);
                return Some(result);
            }
            //se siamo invece all'inizio del path e questo non inizia con D:/ allora ritorna errore
           if dir != curr_dir.name && index== 0 {
                    println!("Directory non trovata in search_parent");
                    return None;
            //se siamo invece all'inizio del path e questo inizia con D:/ allora prosegui in avanti nel path
            }else if dir == curr_dir.name && index== 0 {
                index+=1;
                continue;
            }else{ //se invece NON siamo all'inizio del path
                    //leggiamo lungo i figli della directory corrente
                    for child in &(curr_dir.children) {
                        match child {
                            //se il figlio correntemente in analisi è una directory allora leggila
                            Node::Dir(c) => {   
                                                    //se il suo nome coincide con il nome della variabile dir (attuale sottoparte del path in analisi)
                                                    if c.name == dir {
                                                        //allora spostati alla cartella in questione e segnalalo con un flag
                                                        curr_dir = c;
                                                        found = 1;
                                                        break;
                                                    }
                                                },
                            //se il figlio correntemente in analisi è un file non mi interessa
                            Node::File(_) => {
                                                    continue;
                            }
                        }
                        
                    }
                    //se lo studio lungo il vettore di directory non ha portato a nulla allora ritorna errore
                    if found == 0 {
                        println!("Directory non trovata in search_parent");
                        return None;
                    }
                
                index+=1;
            }
        }
        println!("Directory non trovata in mk_dir");
        return None;
    }

    fn mk_dir(self,path : &str) -> Result<&mut Dir>{
        let mut directories : Vec<&str> = path.split('/').collect();
        directories = directories[1..directories.len()-1].to_vec();
        let dir_name = directories[directories.len()-1];
        //ad es D:/a/b/c viene divisa in 'D:' ,'a' ,'b' e 'c'
        //usa una funzione di ricerca che partendo da FileSystem -> root vada a cercare lungo i figli di D: la cartella 'a', lungo i figli di 'a' cerca 'b' e così via
        
        //trova il direttorio padre
        let parent_dir:Option<&Dir> = self.search_parent(directories);

        match parent_dir {
            None  => return Err(anyhow::Error::new(io::Error::new(io::ErrorKind::NotFound, "Directory non trovata in mk_dir"))),
            Some(parent_dir) => {
                                        let dir = Dir::new(dir_name);

                                        parent_dir.children.push(Node::Dir(dir));

                                        //faccio tutto sto macello perchè voglio ritornare un puntatore alla zona di memoria del vettore non alla variabile nello scope corrente
                                        match &mut parent_dir.children[parent_dir.children.len() - 1] {
                                            Node::Dir(dir) => return Ok(dir),
                                            _ => return Err(anyhow::Error::new(io::Error::new(io::ErrorKind::Other, "Il Node ricercato non è una directory"))),
                                        }
                                        
            }
        };
        
        
    }

    fn rm_dir(self,path:&str) -> anyhow::Result<&Dir>{
        let mut directories : Vec<&str> = path.split('/').collect();
        directories = directories[1..directories.len()-1].to_vec();
        let dir_name = directories[directories.len()-1];

        //trova il direttorio padre
        let parent_dir:&Dir = self.search_parent(directories).expect("Impossibile trovare il direttorio padre in rm_dir");
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

    fn new_file(self,path: &str, file: File) -> Result<&mut File> {
        let mut directories : Vec<&str> = path.split('/').collect();
        directories = directories[1..directories.len()-1].to_vec();
        let file_name = directories[directories.len()-1];
        //ad es D:/a/b/c viene divisa in 'D:' ,'a' ,'b' e 'c'
        //usa una funzione di ricerca che partendo da FileSystem -> root vada a cercare lungo i figli di D: la cartella 'a', lungo i figli di 'a' cerca 'b' e così via
        
        //trova il direttorio padre
        let parent_dir:Option<&Dir> = self.search_parent(directories);

        match parent_dir {
            None  => return Err(anyhow::Error::new(io::Error::new(io::ErrorKind::NotFound, "Directory non trovata in new_file"))),
            Some(parent_dir) => {
                                        

                                        parent_dir.children.push(Node::File(file));

                                        //faccio tutto sto macello perchè voglio ritornare un puntatore alla zona di memoria del vettore non alla variabile nello scope corrente
                                        match &mut parent_dir.children[parent_dir.children.len() - 1] {
                                            Node::File(file) => return Ok(file),
                                            _ => return Err(anyhow::Error::new(io::Error::new(io::ErrorKind::Other, "Il node ricercato in new_file non è un file"))),
                                        }
                                        
            }
        };
        
    }

    fn rm_file(self,path:&str) -> anyhow::Result<&Dir>{
        let mut directories : Vec<&str> = path.split('/').collect();
        directories = directories[1..directories.len()-1].to_vec();
        let file_name = directories[directories.len()-1];

        //trova il direttorio padre
        let parent_dir = self.search_parent(directories);
        let mut index = 0;

        match parent_dir {
            None  => return Err(anyhow::Error::new(io::Error::new(io::ErrorKind::NotFound, "Directory non trovata in new_file"))),
            Some(parent_dir) => {
                                        
                for child in &mut parent_dir.children {

                    match child{
                        Node::File(file) => {
                            if file.name == file_name{
                                parent_dir.children.remove(index);
                                return Ok(parent_dir);
                            }
                        }
                        Node::Dir(_)=> continue 
                    }
                    index+=1;
                }
                return Err(anyhow::Error::new(io::Error::new(io::ErrorKind::NotFound, "Directory non trovata in new_file")));
                                        
            }
        };


       
        
        

    }


}

    


fn main() {
    println!("Hello, world!");
}
