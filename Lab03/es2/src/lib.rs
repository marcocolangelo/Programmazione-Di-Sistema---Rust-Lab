use std::sync::Mutex;

pub struct RingState<T>{
    buf : Vec<T>,
    read : usize,       //indice puntatore ultimo oggetto letto
    write : usize,      //indice puntatore ultimo oggetto scritto
    full : bool         //il buffer è pieno? Ovvero non c'è più spazio per scrivere
}

impl<T> RingState<T> where T: Default + Copy{
    pub fn new(cap:usize) -> Self{
        RingState { buf: vec![T::default();cap], write:0 ,read: 0, full: false }
    }
} 

pub struct RingBuf<T>{
    state : Mutex<RingState<T>>     //rendo RingBuf come semaforo Mutex allo stato del buffer
}

impl<T> RingBuf<T> where T:Default + Copy{
    pub fn new(cap: usize) -> Self{
        RingBuf { state: Mutex::new(RingState::new(cap)) }
    }

    pub fn read(&self) -> Option<T>{
        let mut state = self.state.lock().unwrap();
        if state.write == state.read && state.full != true{    
            println!("Buffer vuoto!");
            return None
        }else{
            let index = state.read;
            let res = state.buf[index].clone(); //non sappiamo che oggetto sia T ma sappiamo che è clonabile perchè implementa Copy
            state.read = (state.read + 1) % state.buf.len();    //poichè circolare dobbimo usare l'operatore modulo
            state.full = false; //se abbiamo letto significa che abbiamo liberato uno spazio dunque non può essere pieno il buffer
            return Some(res);
        }

    } 

    pub fn write(&self, elem : T) -> Result<(),()>{
        let mut state = self.state.lock().unwrap();
        if state.full{
            println!("Buffer pieno!");
            return Err(());
        }else{
            let index = state.write;
            state.buf[index] = elem.clone();
            state.write = (state.write + 1) % state.buf.len();
            if state.write == state.read {      //se una volta aggiornato il contatore arrivo a dire che i due counter sono uguali significa che ho raggiunto l'altro capo del buffer, ergo l'ho riempito
                state.full = true;
            } 
            return Ok(())
        }
    }
}

