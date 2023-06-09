use std::cell::{RefCell};
use std::ops::DerefMut;
use std::rc::Rc;
use std::{
    option::Option,
    time::{SystemTime, UNIX_EPOCH},
};

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
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        File {
            name: String::from(name),
            content,
            creation_time: t,
            type_,
        }
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
    File(Rc<RefCell<File>>),
    Dir(Rc<RefCell<Dir>>),
}

impl Node {

    // ***** this makes the trick: we can clone a node and get a new reference to the same object
    // this allow to easily copy nodes while searching and for storing objects
    // this is made possible by the use of Rc and RefCell
    pub fn clone(&self) -> Node {
        match self {
            Node::File(f) => Node::File(f.clone()),
            Node::Dir(d) => Node::Dir(d.clone()),
        }
    }
}


pub struct MatchResult<'a> {
    qs: Vec<&'a str>,
    matched_nodes: Vec<Node>,
}
#[derive(Debug)]
pub struct Filesystem {
    root: Node,
}

impl Filesystem {
    pub fn new() -> Self {
        Filesystem {
            root: Node::Dir(Rc::new(RefCell::new(Dir::new(String::from(""))))),
        }
    }

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

        let mut result = String::from("");
        for p in parts.iter() {
            if *p == "" {
                continue;
            }
            result.push_str("/");
            result.push_str(p);
        }
        if result == "" {
            result.push_str("/");
        }

        Some((result, String::from(last)))
    }

    pub fn _find(path: &[&str], node: &Node) -> Option<Node> {
        
        match node {
            Node::Dir(d) => {
                let d = &*d.borrow();
                let found = d.children.iter().find(|x| match x {
                    Node::Dir(d) => d.borrow().name == path[0],
                    Node::File(f) => f.borrow().name == path[0],
                });

                match found {
                    Some(x) => {
                        if path.len() == 1 {
                            Some(x.clone())
                        } else {
                            Filesystem::_find(&path[1..], x)
                        }
                    }
                    _ => None,
                }
            }
            Node::File(_) => {
                if path.len() == 1 {
                    Some(node.clone())
                } else {
                    None
                }
            }
        }

    }

    // return a node if it exists; it may be either a file or a directory
    pub fn find_node(&self, path: &str) -> Option<Node> {

        let parts = if path == "/" {vec![""]} else { path.split("/").collect::<Vec<&str>>()};

        if parts.len() < 1 {
            return None;
        }

        if parts.len() == 1 && parts[0] == "" {
            return Some(self.root.clone());
        }

        return Filesystem::_find(&parts[1..], &self.root);
    }


    fn new_node(&mut self, path: &str, node: Node) -> Option<Node> {
        let parent = self.find_node(&path);
        
        return match parent {
            Some(x) => {
                if let Node::Dir(d) = x {
                    let mut dir = d.borrow_mut();
                    dir.deref_mut().children.push(node.clone());
                }
                Some(node)
            }
            None => None, // adding to a file
        };
    }

    pub fn mk_dir(&mut self, path: &str) -> Option<Node> {
        if let Some((path, last)) = Filesystem::split_path(path) {
            
            let dir = Rc::new(RefCell::new(Dir::new(String::from(last))));
            return self.new_node(&path, Node::Dir(dir));
        }
        None // invalid path
    }

    // it works either for file and dir
    pub fn rm_node(&mut self, path: &str) {
        if let Some((path, last)) = Filesystem::split_path(path) {
            let parent = self.find_node(&path);
            match parent {
                Some(x) => {
                    if let Node::Dir(dref) = x {
                        let mut d = dref.borrow_mut();
                        d.deref_mut().children.retain(|n| match n {
                            Node::Dir(d) => d.borrow().name != last,
                            Node::File(f) => f.borrow().name != last,
                        });
                    }
                }
                None => (),
            }
        }
    }

    pub fn new_file(&mut self, path: &str, file: File) -> Option<()> {
        
        let file_ref = Rc::new(RefCell::new(file));
        self.new_node(path, Node::File(file_ref)).map(|_| ())
    }

    pub fn get_dir(&self, path: &str) -> Option<Rc<RefCell<Dir>>> {
        let node = self.find_node(&path);
        if let Some(x) = node {
            if let Node::Dir(d) = x {
                return Some(d.clone());
            }
        }
        None
    }

    pub fn get_file(&self, path: &str) -> Option<Rc<RefCell<File>>> {
        let node = self.find_node(&path);
        if let Some(x) = node {
            if let Node::File(f) = x {
                return Some(f.clone());
            }
        }
        None
    }

    fn do_match<'a>(node: &Node, qs: &'a [&'a str]) -> Option<Vec<&'a str>> {
        let mut matched = vec![];

        for q in qs {
            let toks = q.split(":").collect::<Vec<&str>>();
            let qtype = toks[0];
            let qval = toks[1];
            match qtype {
                "name" => match node {
                    Node::Dir(x) => {
                        if x.borrow().name.contains(&qval) {
                            matched.push(*q);
                        }
                    }
                    Node::File(f) => {
                        if f.borrow().name.contains(&qval) {
                            matched.push(*q);
                        }
                    }
                },
                // TODO: add here other matches
                _ => println!("'{}' unknown or unhandled qtype", qtype),
            }
        }
        if matched.len() == 0 {
            return None;
        }
        Some(matched)
    }

    pub fn search<'a>(&'a mut self, qs: &'a [&'a str]) -> Option<MatchResult> {
        let mut mr = MatchResult {
            qs: vec![],
            matched_nodes: vec![],
        };

        let mut visits = vec![self.root.clone()];

        while let Some(node) = visits.pop() {

            if let Node::Dir(d) = node.clone() {

                for cc in d.borrow().children.iter() {
                    if let Node::Dir(_) = cc {
                        visits.push(cc.clone());
                    }

                    if let Some(matches) = Filesystem::do_match(cc, qs) {
                        for m in matches {
                            if !mr.qs.contains(&m) {
                                mr.qs.push(m);
                            }
                        }
                        mr.matched_nodes.push(node.clone());
                    };
                }
            }
        }
        Some(mr)
    }

    pub fn print(&mut self) {
        let mut visits = vec![self.root.clone()];

        while let Some(node) = visits.pop() {
            if let Node::Dir(d) = node {
                
                println!("dir: {}", d.borrow().name);

                for cc in d.borrow().children.iter() {
                    if let Node::Dir(_) = cc {
                        visits.push(cc.clone());
                    }
                    if let Node::File(f) = cc {
                        println!("file: {}/{}", d.borrow().name, f.borrow().name);
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
    fn split_path() {
        let (path, last) = Filesystem::split_path("/home").unwrap();
        assert_eq!(path, "/");
        assert_eq!(last, "home");
        
        let (path, last) = Filesystem::split_path("/home/me").unwrap();
        assert_eq!(path, "/home");
        assert_eq!(last, "me");
    }

    #[test]
    fn create_empty_fs() {
        let fs = Filesystem::new();
        if let Node::Dir(d) = fs.root {
            assert_eq!(d.borrow().name, "");
        } else {
            assert!(false, "expected dir")
        }
    }

    #[test]
    fn create_dir() {
        let mut fs = Filesystem::new();
        fs.mk_dir("/home");

        if let Node::Dir(d) = &fs.root {
            let d = d.borrow();
            assert_eq!(d.children.len(), 1);
            
            if let Node::Dir(d) = &d.children[0] {
                assert_eq!(d.borrow().name, "home");
            } else {
                assert!(false, "expected dir")
            }
        } else {
            assert!(false, "expected dir")
        }
    }

    #[test]
    fn create_multiple_dirs() {
        
        let fs = mk_fs();
        
        if let Node::Dir(ref root) = &fs.root {

            let root = root.borrow();
            assert_eq!(root.children.len(), 1);
            assert_eq!(root.name, "");

            if let Node::Dir(home) = &root.children[0] {
                let home = home.borrow();
                assert_eq!(home.children.len(), 1);
            
                if let Node::Dir(me) = &home.children[0] {
                    let me = me.borrow();
                    assert_eq!(me.name, "me");
                } else {
                    assert!(false, "expected dir")
                }
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
        fs.rm_node("/home/me");
        
        if let Node::Dir(root) = fs.root.clone() {
            assert_eq!(root.borrow().children.len(), 1);
        }
        fs.rm_node("/home");
        
        if let Node::Dir(root) = fs.root.clone() {
            assert_eq!(root.borrow().children.len(), 0);
        }
    }

    #[test]
    fn invalid_paths() {
        let mut fs = Filesystem::new();
        
        match fs.mk_dir("home") {
            Some(_) => assert!(false, "expected None"),
            None => (),
        };

        if let Node::Dir(root) = fs.root {
            assert_eq!(root.borrow().children.len(), 0);
        }
    }

    #[test]
    fn add_file() {
        let mut fs = mk_fs();
        fs.new_file("/", File::new("test", "my content".into(), FileType::Text));
        
        if let Node::Dir(root) = &fs.root {
            assert_eq!(root.borrow().children.len(), 2);
        }
        
        fs.new_file(
            "/home",
            File::new("testf", "my content".into(), FileType::Text),
        );

        let dir = fs.get_dir("/home").unwrap();
        assert_eq!(dir.borrow().children.len(), 2);
    }

    #[test]
    fn match_files() {
        let mut fs = mk_fs();
        
        fs.new_file("/", File::new("test1", "my content".into(), FileType::Text));
        fs.new_file(
            "/home",
            File::new("test2", "my content".into(), FileType::Text),
        );

        let mr = fs.search(&["name:test1"]).unwrap();
        assert_eq!(mr.qs.len(), 1);
        assert_eq!(mr.matched_nodes.len(), 1);
        let mr = fs.search(&["name:test"]).unwrap();
        assert_eq!(mr.qs.len(), 1);
        assert_eq!(mr.matched_nodes.len(), 2);
        let mr = fs.search(&["name:nomatch"]).unwrap();
        assert_eq!(mr.qs.len(), 0);
        assert_eq!(mr.matched_nodes.len(), 0);
    }

    #[test]
    fn match_files_and_dirs() {
        let mut fs = mk_fs();

        fs.new_file(
            "/home",
            File::new("home", "my content".into(), FileType::Text),
        );

        let mr = fs.search(&["name:home"]).unwrap();
        assert_eq!(mr.qs.len(), 1);
        assert_eq!(mr.matched_nodes.len(), 2);
        let mr = fs.search(&["name:me"]).unwrap();
        assert_eq!(mr.qs.len(), 1);
        assert_eq!(mr.matched_nodes.len(), 3);
        let mr = fs.search(&["name:nomatch"]).unwrap();
        assert_eq!(mr.qs.len(), 0);
        assert_eq!(mr.matched_nodes.len(), 0);
    }
}
