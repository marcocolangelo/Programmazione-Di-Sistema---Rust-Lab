
pub struct Prova{
    
    nome:String,
    numero:i32
}


impl Prova{
    fn prova_clone(self) -> Self {Self{nome:self.nome,numero:self.numero+1}}
    fn prova_copy(&self) -> i32 {self.numero}
    fn prova_write(&mut self,num:i32){self.numero=num}
}

fn main(){
    let s1 = Prova{nome:"ciao".to_string(),numero:11};
    let mut s2 = Prova{..s1};
    
    let s4=s1.prova_clone();

    //println!("{}",s1.prova_copy());

    let mut s4= Prova{..s2};

    s4.prova_write(121);

    println!("{}",s1.numero);

}