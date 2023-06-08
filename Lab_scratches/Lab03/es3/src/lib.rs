use std::{option::Option, time::{SystemTime, UNIX_EPOCH}, cell::RefCell};

#[derive(Debug)]
pub enum FileType {
    Text,
    Binary,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub content: Vec<u8>,
    pub creation_time: u64,
    pub type_: FileType,
}

impl File {
    pub fn new(name: &str, content: Vec<u8>, type_: FileType) -> Self {
        let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        File { name: String::from(name), content, creation_time: t, type_ }
    }
}

#[derive(Debug)]
pub struct Dir {
    pub name: String,
    pub creation_time: u64,
    children: Vec<Node>,
}

impl Dir {
    pub fn new(name: String) -> Self {
        Dir {
            name,
            creation_time: 0,
            children: vec![],
        }
    }
}

#[derive(Debug)]
pub enum Node {
    File(File),
    Dir(Dir),
}

#[derive(Debug)]
pub struct Filesystem {
    root: Dir,
}

pub struct MatchResult<'a> {
    qs: Vec<&'a str>, //queries matchate che vanno applicate in OR
    matched_nodes: Vec<RefCell<&'a Node>>,   //risultati delle queries, ora devi gestire anche riferimenti mutabili alle cartelle
}


impl Filesystem {
    pub fn new() -> Self {
        Filesystem {
            root: Dir::new(String::from("")),
        }
    }

//verifica che il path passato sia legale
    fn check_path(path: &str) -> bool {
        let parts = path.split("/").collect::<Vec<&str>>();
        if parts.len() < 2 || parts[0] != "" {
            return false;
        }
        true
    }

    // accepting only absolute paths
    fn split_path(path: &str) -> Option<(String, String)> {
        if !Filesystem::check_path(path) {
            return None;
        }
        let mut parts = path.split("/").collect::<Vec<&str>>();
        let last = parts.remove(parts.len() - 1);
        Some((parts.join("/"), String::from(last)))
    }


    //trova una cartella dato il suo path
    fn find_dir(&mut self, path: &str) -> Option<&mut Dir> {
        let parts = path.split("/").collect::<Vec<&str>>();
        if parts.len() < 1 {
            return None
        }
        
        let mut cdir = &mut self.root;
        if parts.len() == 1 && parts[0] == "" {
            return Some(cdir);
        }

        //il ciclo si conclude quando ovviamente sei alla fine del path
        for part in parts.iter() {
            if *part == "" {
                continue;
            }

            //itera lungo tutti i figli della cartella corrente e verifica quali di questi è quello che coincide con la parte corrente del path in analisi
            let n = cdir.children.iter_mut().find(|x| match x {
                Node::Dir(x) => x.name == *part,
                _ => false,
            });

            //se ne hai trovato uno allora rendi questa cartella la cartella da analizzare per la prossima iterazione
            match n {
                Some(Node::Dir(ref mut x)) => {
                    cdir = x;
                }
                _ => return None,
            }
        }

        //ritorna l'ultima cartella trovata durante lo svolgimento del path(la cartella che ha come nome l'ultimo pezzo del path)
        Some(cdir)
        
    }

    pub fn mk_dir(&mut self, path: &str) -> Option<&mut Dir> {
        if let Some((path, last)) = Filesystem::split_path(path) {
            let parent = self.find_dir(&path);
            return match parent {
                Some(x) => {
                    let new_dir = Dir::new(String::from(last));
                    x.children.push(Node::Dir(new_dir));
                    Some(x)
                }
                None => None,
            };
        }
        None
    }

    pub fn rm_dir(&mut self, path: &str) {
        if let Some((path, last)) = Filesystem::split_path(path) {
            let parent = self.find_dir(&path);
            match parent {
                Some(x) => {
                    x.children.retain(|x| match x {
                        Node::Dir(x) => x.name != last,
                        _ => true,
                    });
                }
                None => (),
            }
        }
    }

    pub fn new_file(&mut self, path: &str, file: File) -> Option<()> {
        let d = self.find_dir(path);
        match d {
            Some(x) => {
                x.children.push(Node::File(file));
                Some(())
            }
            None => None,
        }
    }

    pub fn rm_file(&mut self, path: &str) {
        if let Some((path, last)) = Filesystem::split_path(path) {
            let parent = self.find_dir(&path);
            match parent {
                Some(x) => {
                    x.children.retain(|x| match x {
                        Node::File(x) => x.name != last,
                        _ => true,
                    });
                }
                None => (),
            }
        };
    }

    pub fn get_file(&mut self, path: &str) -> Option<&mut File> {
        if let Some((path, last)) = Filesystem::split_path(path) {
            let parent = self.find_dir(&path);
            return match parent {
                Some(x) => {
                    let f = x.children.iter_mut().find(|x| match x {
                        Node::File(x) => x.name == last,
                        _ => false,
                    });
                    match f {
                        Some(Node::File(ref mut x)) => Some(x),
                        _ => None,
                    }
                }
                None => None,
            }
        }
        None
    }

//la funzione ritorna la somma della dimensione del contenuto dei file della cartella se n è una Dir, altrimenti direttamente
    //la lunghezza del contenuto del file se n è File 
    fn get_content (n : RefCell<&Node>) -> i32{
        let node = n.borrow_mut();
        match &*node {
            Node::Dir(d) => {
                if d.children.len() == 0{
                    return 0;
                }
                d.children.iter().map(|nc| Filesystem::get_content(RefCell::new(nc))).sum()
            }
            Node::File(f) => {
                f.content.len() as i32
            }
        }
    }

//wrapper per raccogliere le funzioni atte ad eseguire tutte le possibili queries
    fn do_match<'a>(n: RefCell<&Node>, qs: &'a[&'a str]) -> Option<Vec<&'a str>> {           
        let mut matched = vec![];
        let node = n.borrow_mut();
        for q in qs {               //affronta una richiesta alla volta
            let toks = q.split(":").collect::<Vec<&str>>(); //splitta la query in spezzoni TIPO della query e ARGOMENTO del comando
            let qtype = toks[0];
            let qval = toks[1];
            match qtype {
                "name" => {         //se la query è "name:stringa"
                    match *node{
                        Node::Dir(d) => if d.name.contains(&qval) {
                            matched.push(*q);
                        }
                        Node::File(f) => if f.name.contains(&qval) {
                            matched.push(*q);
                    }
                        
                    }
                }
                "content" => {
                    match *node{
                        Node::File(f) => {
                            let contenuto = f.content.windows(qval.as_bytes().len()).any(|w| w==qval.as_bytes());
                            if contenuto{
                                matched.push(*q);
                            }
                        }
                        Node::Dir(_) => continue,
                    }
                }
                "larger" => {
                    let sum = Filesystem::get_content(RefCell::new(&*node));
                    if sum > qval.parse().unwrap(){
                        matched.push(*q);
                    }
                    
                }
                "smaller" => {
                    let sum = Filesystem::get_content(n.clone());
                    if sum > qval.parse().unwrap(){
                        matched.push(*q);
                    }
                }

                // TODO: add here other matches
                _ => println!("'{}' unknown or unhandled qtype", qtype),
            }
        }
        if matched.len() == 0 {         //se non ho ottenuto risultati da alcuna query lo segnalo
            return None;
        }
        Some(matched)
    }

//cerca tra tutte le cartelle e File quali di questi ritornano dei risultati dalle varie queries passate come input 
    pub fn search<'a>(&'a mut self, qs: &'a [&'a str]) -> Option<MatchResult> {
        
        let mut mr = MatchResult {
            qs: vec![],         //vettore delle queries matchate
            matched_nodes: vec![],  //vettore di Nodi ritornati come risultati delle queries matchate
        };

        let mut visits = vec![&self.root];          //vettore con riferimenti alle cartelle da visitare (inizia con root ovviamente)
        while let Some(d) = visits.pop() {                      //finchè c'è una cartella da visitare continua
            for cc in d.children.iter() {                       //usa iter_mut()  per iterare lungo tutti i figli della cartella correntemente in analisi
                //let ref_cc = RefCell::new(cc);
                
                    match cc {
                        
                        Node::Dir( x) => {
                            if let Some(matches) = Filesystem::do_match(RefCell::new(cc), qs){
                            for m in matches  {
                                if !mr.qs.contains(&m) {        //verifica se questa query non è gia stata affrontata prima
                                    mr.qs.push(m);                  //se no, inseriscila nel vettore
                                }
                            }
                            mr.matched_nodes.push(RefCell::new(cc)); 
                            visits.push(x);
                            }else{
                                visits.push(x);
                            }
                        }
                        
                        Node::File(_) => {
                            if let Some(matches) = Filesystem::do_match(RefCell::new(cc), qs){
                            for m in matches {
                                if !mr.qs.contains(&m) {        //verifica se questa query non è gia stata affrontata prima
                                    mr.qs.push(m);                  //se no, inseriscila nel vettore
                                }
                            }
                            mr.matched_nodes.push(RefCell::new(cc));          //inserimento del Node trovato nel vettore di nodi del MatchResult
                        
                            }
                         }
                    }
                
                    
            }
        }
        Some(mr)
    }
        
    

    pub fn print(&mut self) {
        let mut visits = vec![&mut self.root];
        while let Some(d) = visits.pop() {
            for cc in d.children.iter_mut() {
                //let cc = &mut *c;
                match cc {
                    Node::Dir(ref mut x) => {
                        println!("dir: {}/{}", d.name, x.name);
                        visits.push(x);
                    }
                    Node::File(x) => {
                        println!("file: {}/{}", d.name, x.name);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_fs() -> Filesystem {
        let mut fs = Filesystem::new();
        fs.mk_dir("/home");
        fs.mk_dir("/home/me");
        fs
    }

    #[test]
    fn create_empty_fs() {
        let fs = Filesystem::new();
        assert_eq!(fs.root.name, "");
    }

    #[test]
    fn create_dir() {
        let mut fs = Filesystem::new();
        fs.mk_dir("/home");
        assert_eq!(fs.root.children.len(), 1);
        if let Node::Dir(ref d) = fs.root.children[0] {
            assert_eq!(d.name, "home");
        } else {
            assert!(false, "expected dir")
        }
    }

    #[test]
    fn create_multiple_dirs() {
        let mut fs = mk_fs();
        assert_eq!(fs.root.children.len(), 1);

        if let Node::Dir(ref d) = fs.root.children[0] {
            assert_eq!(d.name, "home");
            assert_eq!(d.children.len(), 1);
            if let Node::Dir(ref d) = d.children[0] {
                assert_eq!(d.name, "me");
            } else {
                assert!(false, "expected dir")
            }
        } else {
            assert!(false, "expected dir")
        }
    }

    #[test]
    fn rm_dir() {
        let mut fs = mk_fs();
        fs.rm_dir("/home/me");
        assert_eq!(fs.root.children.len(), 1);
        fs.rm_dir("/home");
        assert_eq!(fs.root.children.len(), 0);

    }

    #[test]
    fn invalid_paths() {
        let mut fs = Filesystem::new();
        match fs.mk_dir("home") {
            Some(_) => assert!(false, "expected None"),
            None => (),
        };

        assert_eq!(fs.root.children.len(), 0);
    } 

    #[test]
    fn add_file() {
        let mut fs = mk_fs();
        fs.new_file("/", File::new("test", "my content".into(), FileType::Text ));
        assert_eq!(fs.root.children.len(), 2);
        fs.new_file("/home", File::new("test", "my content".into(), FileType::Text ));
        if let Node::Dir(ref d) = fs.root.children[0] {
            assert_eq!(d.children.len(), 2);
        } else {
            assert!(false, "expected dir")
        }
    }

    #[test]
    fn match_files() {
        
        let mut fs = mk_fs();
        fs.new_file("/", File::new("test1", "my content".into(), FileType::Text ));
        assert_eq!(fs.root.children.len(), 2);
        fs.new_file("/home", File::new("test2", "my content".into(), FileType::Text ));

        let mr = fs.search(&["name:test1"]).unwrap();
        assert_eq!(mr.qs.len(), 1);
        assert_eq!(mr.matched_nodes.len(), 1);
        let mr = fs.search(&["name:test"]).unwrap();
        assert_eq!(mr.qs.len(), 1);
        assert_eq!(mr.matched_nodes.len(), 2);
        let mr = fs.search(&["name:home"]).unwrap();
        assert_eq!(mr.qs.len(), 1);
        assert_eq!(mr.matched_nodes.len(), 1);

    }

}
