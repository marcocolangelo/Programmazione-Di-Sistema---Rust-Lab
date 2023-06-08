//producer
use serde::{Serialize, Deserialize};
use fslock::LockFile;
use rand::{Rng};
use std::fs::{File};
use std::io::{Write, Read,Seek};
use std::fs::OpenOptions;
use std::time::{UNIX_EPOCH};
use std::{thread, time, mem};
use std::io::{self};
use std::mem::size_of_val;





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
#[derive(Serialize, Deserialize,Debug)]
pub struct SensorData {
    seq: u32, // sequenza letture
    values: [f32; 10],
    timestamp: u32
}

//struttura per l'header del file buffer circolare
#[repr(C)]
#[derive(Serialize, Deserialize,Debug)]
pub struct Header{
    read_index : u32,
    write_index : u32,
    buffer_size : u32
}

//implemento Default per l'inizializzazione di SensorData 
impl Default for SensorData{
    fn default() -> Self {
        Self { seq: 0, values: [0.0;10], timestamp: 0 }
    }
}

impl Default for Header{
    fn default() -> Self {
        Self {read_index :0,write_index:0,buffer_size:10}
    }
}

impl Header {
    fn set_read_index(&mut self ,new_value : u32) -> (){
        self.write_index = new_value;
    }
}



//ATTENTO CHE VIENE IMPLEMENTATO TUTTO TRAMITE PUNTATORI, VERIFICA PER BENE LORO FUNZIONAMENTO
fn random_production(sensordata:&mut SensorData, header:&Header) -> () {
    let mut rng = rand::thread_rng();
    

    //genera i dati sei sensori fake tramite generazione di valori randomici
    (*sensordata).values = [rng.gen();10];
    (*sensordata).seq = header.write_index;
    (*sensordata).timestamp = std::time::SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as u32;
    
    /*
         --SOLO A SCOPO DI DEBUG--
    println!("{:?}",(*sensordata).values);
    */
}

//la funzione è di tipo GENERICO per essere usata sia con Header che con SensorData
fn write_on_file <T> (data_file : &mut File,offset :u64, data: & T  ) -> ()
    where T:Serialize
{
    //conversione della struttura in binario
    let data_buf = bincode::serialize( data).unwrap();
    //println!("Dato serializzato : {:?}",data_buf);
    //posizionamento puntatore nella giusta posizione
    data_file.seek(std::io::SeekFrom::Start(offset)).unwrap(); 
    
    //scrivi il binario del sensordata su file
    (*data_file)
        .write(&data_buf)
        .expect("Scrittura fallita nella funzione write_on_file del producer(LockFile,DataFile)");
}

fn read_from_file <'a,'b,T> (data_file : &'a mut File,offset :u64, buffer : &'b mut Vec<u8> ) ->  T where T:Deserialize<'b>
{
    
    //let mut buf:Vec< u8> = vec![0;size_of_val(&data)+1];

    data_file.seek(std::io::SeekFrom::Start(offset)).unwrap(); 
    //lettura dell'header del file per capire qual è la situazione attuale del buffer
    let res = data_file.read(buffer);
    //verifica se c'è stato qualche errore di lettura
    if res.is_err() {
        println!("Errore lettura header nel producer!");
    }else{
        println!("Bytes letti da read_from_file : {:?}",res);
    }

    //riporto l'header da forma binaria a forma originaria per leggerne i dati utili alla scrittura
    return bincode::deserialize(buffer).unwrap();
  
}

fn produce(data_file:&mut File) -> Result<(), fslock::Error>{
    let mut sensordata: SensorData = SensorData::default();
    let mut header : Header = Header::default();
    let size_of_head = size_of_val(&header);
   
    //buffer su misura per Header così da poter scrivere e leggere correttamente l'header ad ogni chiamata del producere e consumer
    let mut header_buf:Vec<u8> = vec![0;size_of_head+1];
    
    //lettura header per estrapolare stato corrente del buffer
    header = read_from_file(data_file, 0, &mut header_buf);

    let write_index = header.write_index;
    let buffer_size=header.buffer_size;
    let read_index = header.read_index;

    println!("write_index : {:?}, buffer_size : {:?}, read_index : {:?}",write_index,buffer_size,read_index);

    //produce i dati dei fake sensori
    random_production(&mut sensordata,&header);

    //verifica che il buffer non sia pieno (write_index > buffer_size) e nel caso in cui lo fosse, verifica se puoi riprendere da 0 ( cioè vedi se read_index > 0)
    if write_index > buffer_size &&  read_index == 0{
        println!("Buffer pieno, permetti al consumer di leggere primo elemento ");
        return Ok(());
    } else if write_index > buffer_size && read_index > 0{
        println!("Buffer pieno, ma possibile scrittura ricominciando da 0");
    }else{
        println!("Scrittura su {:?} e indice lettura su {:?}",write_index,read_index);
    }
    
    //rendo binario la struttura SensorData solo per calcolarne la dimensione per l'offset
    let mut sensor_buf = bincode::serialize(&sensordata).unwrap(); //questo serve solo per calcolare valore offset
    let offset = mem::size_of_val(&header_buf) as u32 + (mem::size_of_val(&sensor_buf) as u32)*write_index;

    //scrivi in binario la struttura dati SensorData nella posizione write_index del buffer (offset = offset as u64)
    write_on_file(data_file, offset as u64, &sensordata);

    sensordata = read_from_file(data_file, offset as u64,&mut sensor_buf );

    println!("DEBUG : Lettura buffer_circolare al write_index {:?} : sensordata -> {:?} ",write_index,&sensordata);

    //aggiorna puntatore per la scrittura
    header.set_read_index(write_index+1);

    //scrivi in binario la struttura Header data nel file come INTESTAZIONE INIZIALE (offset=0)
    write_on_file(data_file, 0, &header);

    
    
    Ok(())
}

fn open_or_create_file(filename: &str, existing : &mut i32) -> io::Result<File> {
    //usa match per verificare se il file è già esistente, altrimenti lo creo
    match OpenOptions::new()
        .write(true)
        .read(true)
        .open(filename)
    {
        Ok(file) => { 
            println!("Trovato file {} esistente, apertura in corso...", filename);
            *existing = 1; 
            return Ok(file)
        },
        Err(_) => {
            println!("Il file {} non esiste, creazione in corso...", filename);
            let file = OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .open(filename)?;
            *existing=0;
            Ok(file)
        }
    }
}

fn main()  -> Result<(),fslock::Error>{
    let mut exist = 0;
    let mut result: Result<(), fslock::Error> ;
    //apre il file di lock, separato dal file su cui scrivere
    let mut lock_file = LockFile::open("D:\\Desktop\\I ANNO LM\\II SEMESTRE\\Programmazione di sistema\\PS-Rust\\Lab\\Lab02\\lock_file")?;
    //inizializzo Header a default per la prima lettura/scrittura
    let header : Header = Header::default();
    
    lock_file.lock()?;
    //apri il file di buffer se esiste, altrimenti crealo 
    let mut data_file = open_or_create_file("buffer_circolare.txt",&mut exist).unwrap();
    
    //se il file è stato appena creato allora inserisci l'header inizializzato alle condizioni iniziali
    if exist==0{
        println!("Inizializzazione buffer : scrittura header inizializzato a condizioni di default {:?}",&header);
        write_on_file(&mut data_file, 0, &header);
    }
    lock_file.unlock()?;

    //richiama funzione del produttore in loop, fino a che non interrotto da terminale
    loop{
        lock_file.lock()?;
        
        result = produce(&mut data_file);
        
        lock_file.unlock()?;
        
        //sleep di 1 secondo
        let one_sec = time::Duration::from_millis(1000);
        thread::sleep(one_sec);

        result?
    }

}