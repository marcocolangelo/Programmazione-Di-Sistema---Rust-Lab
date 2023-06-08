/****
1) una barriera è una struttura che permette di gestire un flusso di thread
    Essa ha due stati di funzionamento: ATTESA ovvero cbarrier.wait() oppure SVUOTAMENTO quando thread ci passano attraverso
2) i thread possono passare attraverso la barriera solo se sono tutti insieme 
    (cioè se dato N il numero di thread totali e K numero di thread presso la barriera allora K=N)  
3) Si dice CICLICA una barriera che può essere usata più volte da un thread (ma che rispetta le classiche regole di una barriera)
4) E' possibile che thread molto veloci passino una seconda volta attraverso la barriera mentre altri stanno ancora passando la prima 
    (ovvero mentre la barriera è ancora aperta per la prima volta)
5) Serve perciò anche uno STATO (APERTA/CHIUSA) oltre al CONTATORE(K thread in attesa) 
****/

use std::sync::{Mutex, Condvar,MutexGuard,LockResult};
use crossbeam::channel::{bounded,Receiver,Sender};


/** IMPLEMENTAZIONE 1   -   MUTEX + CONDITION VARIABLE per la gestione dello STATO CONDIVISO **/
pub struct CBState{     //struttura da mettere nel Mutex di CBclassic
    pub contatore : i32,    //contatore di quanti thread dentro 
    pub opened : bool,   //stato della barriera : false se chiuso, true se aperto
}

pub struct CBclassic {      //wrapper della barriera
    state : Mutex<CBState>,     //stato barriera -> se non fosse un mutex unico non riusciresti a metterlo dentro cv.wait_while(state,cond)
    cv  :Condvar,       //la condition variable per bloccare il thread fino al triggering di una condizione
    size : i32      //numero di thread avviati
}


impl CBState{
    pub fn new() -> Self{
        CBState { contatore: 0, opened: true }
    }
}

impl CBclassic {
    pub fn new(size:i32) -> Self{
        CBclassic { state:Mutex::new(CBState::new()), cv: Condvar::new(), size : size}
    }

    pub fn wait(&self) {
        let mut state = self.state.lock().unwrap();
        state = self.cv.wait_while(state, |s| s.opened == false).unwrap(); //aspetta che lo stato non sia opened

        if state.opened{
            if state.contatore == self.size - 1{     //deve entrare solo l'ultimo che poi chiude la porta
                state.opened = false;
                self.cv.notify_all();
            } else{
                state.contatore +=  1;
                state = self.cv.wait_while(state, |s| s.opened == true).unwrap();
                if state.opened == false{
                    if state.contatore == 1{     //deve uscire solo uno che poi lascia aperta la porta
                        state.opened = true;
                        state.contatore = 0;
                        self.cv.notify_all();
                    }else{
                        state.contatore -=  1;
                    }
                }else{
                    println!("Errore chiusura barriera ciclica");
                }
            }

        }else{
            println!("Errore apertura barriera ciclica!");
        }
    }
}

/* IMPLEMENTAZIONE 2 - USO DI CANALI */
//ciascun thread ha N TOKEN, di cui N-1 da inviare agli altri due thread
//quando ciascun thread torna a N token dopo averne inviati N-1 allora è possibile entrare/uscire
//possibile rendere ogni token identificativo (cioè appartenente a thread 1, thread 2 ...)
//in tal caso si usa wait(i) con i identificativo del thread che ha inviato il token

//usiamo 3 canali Channel(std::mspc::Sender<()>,std::mspc::Receiver<()>) -> I per 1-2, II per 2-3 , III per 3-1


pub struct CBchannel{
    channels : Vec<(Sender<()>,Receiver<()>)>
}

impl CBchannel{ 
    pub fn new(size:usize) -> Self{
        let mut channels =  Vec::new();
        for i in 0..size{
            channels.push(bounded::<()>(size));
        }
        CBchannel { channels : channels }
    }

    pub fn wait(&self,i:usize){
        //invia un messaggio per ogni canale (ovvero 1 ad ogni thread)
        for (c,_) in &self.channels{
            c.send(()).unwrap();
        }

        let (_,r) = &self.channels[i]; //piazza il thread sul suo canale dedicato
       for _ in 0..self.channels.len(){     //aspetta che arrivi un numero di messaggi quanto è il numero di thread
            r.recv().unwrap();
        } 
    }   
}
