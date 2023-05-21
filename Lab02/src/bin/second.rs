//producer
use serde::{Serialize, Deserialize};
use fslock::LockFile;
use rand::Rng;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;


/*
                                     -- ATTENZIONE --
        Va ancora implementata la compatibilità con il buffer circolare in buffer_circolare.txt
        Per ora scrive la struttura binaria senza mezze misure
        1)Leggi le prime tre righe del file per rilevare indice inizio lettura, indice prima scrittura disponibile e dimensione buffer
        2) Adotta il codice use std::fs::File;
                            use std::io::{Seek, SeekFrom};

                                fn main() -> std::io::Result<()> {
                                    let mut file = File::open("foo.txt")?;
                                    file.seek(SeekFrom::Start(n))?;
                                    Ok(())
                                }
                                Dove n in Start(n) rappresenta la posizione dove poter scrivere/leggere che è presa dall'header del file
      3)Gestisci situazione  buffer pieno nel caso del producer: salta scrittura valore corrente e passa alla prossima lettura del consumer 
        e situazione buffer vuoto: consumer non ha abbastanza dati da leggere per cui prende quello che ha e poi fa andare avanti indice lettura 
        (dovrebbe essere il caso in cui il consumer ha trovato un buco nel buffer)
        
 */


//Usa Serialize e Deserialize di serde e poi il tratto dinamico bincode per la trasformazione in binario  
#[repr(C)]
#[derive(Serialize, Deserialize)]
struct SensorData {
    seq: u32, // sequenza letture
    values: [f32; 10],
    timestamp: u32
}


//implemento Default per l'inizializzazione di SensorData 
impl Default for SensorData{
    fn default() -> Self {
        Self { seq: 0, values: [0.0;10], timestamp: 0 }
    }
}

//ATTENTO CHE VIENE IMPLEMENTATO TUTTO TRAMITE PUNTATORI, VERIFICA PER BENE LORO FUNZIONAMENTO
fn random_production(sensordata:&mut SensorData) -> () {
    let mut rng = rand::thread_rng();

    //genera i dati sei sensori fake tramite generazione di valori randomici
    (*sensordata).values = [rng.gen();10];
    
    /*
         --SOLO A SCOPO DI DEBUG--
    println!("{:?}",(*sensordata).values);
    
    */
}

fn produce(lock_file:&mut LockFile, data_file:&mut File) -> Result<(), fslock::Error>{
    let mut sensordata: SensorData = SensorData::default();

    //qui mettiamo il tentativo di lock in un loop: fino a che non lo ottiene, riprova
    while !(*lock_file).lock().is_ok() {}

    //produce i dati dei fake sensori
    random_production(&mut sensordata);

    // Scrivi sul file buffer_circolare.txt
    /*
            QUI DEVI MODIFICARE PER ADATTARLA ALLA STRUTTURA DI BUFFER CIRCOLARE!!!
     */
    let buf = bincode::serialize(&sensordata).unwrap();
    (*data_file)
        .write(&buf)
        .expect("Scrittura fallita nella funzione produce(LockFile,DataFile)");

    (*lock_file).unlock()?;
    
    Ok(())
}
fn main()  -> Result<(),fslock::Error>{
    //apre il file di lock, separato dal file su cui scrivere
    let mut lock_file = LockFile::open("D:\\Desktop\\I ANNO LM\\II SEMESTRE\\Programmazione di sistema\\PS-Rust\\Lab\\Lab02\\lock_file")?;
    
    // Trova il file su cui scrivere
     /*
            QUI DEVI MODIFICARE PER ADATTARLA ALLA STRUTTURA DI BUFFER CIRCOLARE!!!
     */
    let mut data_file = OpenOptions::new()
        .append(true)
        .open("D:\\Desktop\\I ANNO LM\\II SEMESTRE\\Programmazione di sistema\\PS-Rust\\Lab\\Lab02\\buffer_circolare.txt")
        .expect("impossibile leggere il file per la scrittura in producer");

    

    let mut  result;
    
    loop{
        //richiama funzione del produttore in loop, fino a che non interrotto da terminale
        result = produce(&mut lock_file,&mut data_file); 

        //MA QUESTO '?' COSA FA ACCADERE NON SI SA!
        result?
    }

}