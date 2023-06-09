//consumer
use serde::{Serialize, Deserialize};
use fslock::LockFile;
use std::fs::{File};
use std::io::{Write, Read,Seek};
use std::fs::OpenOptions;
use ordered_float::OrderedFloat;
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

pub struct SensorMatrix{
    rows: u32,
    cols : u32,
    data: Vec<Vec<f32>>
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
    /*fn set_write_index(&mut self ,new_value : u32) -> (){
        self.write_index = new_value;
    }*/
    fn set_read_index(&mut self ,new_value : u32) -> (){
        self.read_index = new_value;
    }
}

impl SensorMatrix {
    fn new(rows : u32, cols :u32) -> SensorMatrix{
        SensorMatrix {
            rows,
            cols,
            data: vec![vec![0.0; cols as usize]; rows as usize]
        }
    }

    /*fn get(&self,row :u32,col:u32) -> f32{
        //self.data[(row*self.cols + col) as usize]
        self.data[row as usize][col as usize]
    }*/

    fn get_col(&self,col : u32) -> Vec<f32>{
        let colonna  = self.data.iter().map(|x| x[col as usize]).collect();
        return colonna;
    }

   /*  fn set(&mut self,row:u32,col:u32,val:f32) -> (){
        //self.data.insert((row*self.cols + col) as usize, val);
        self.data[row as usize][col as usize] = val;
    }*/

    fn set_row(&mut self, row:u32, val:Vec<f32>) -> (){
        self.data[row as usize] = val;
    }

    fn means(&self) -> Vec<f32> {
        let mut means:Vec<f32> = Vec::with_capacity(self.cols as usize);
        let mut mean;
        for i in 0..self.cols{
            mean = (self.get_col(i).iter().map(|x| *x).sum::<f32>() as f32) / (self.rows as f32); 
            means.push(mean);
        }

        return means;

    }

    fn min(&self) -> Vec<f32>{
        let mut mins:Vec<f32> = Vec::with_capacity(self.cols as usize);
        let mut min:f32;

        for i in 0..self.cols{
            min = *self.get_col(i).iter().map( |&x| OrderedFloat(x))
                                         .min()
                                         .unwrap_or(OrderedFloat(std::f32::NAN));
            mins.push(min)
        }

        return mins;
    }

    fn max(&self) -> Vec<f32>{
        let mut maxs:Vec<f32> = Vec::with_capacity(self.cols as usize);
        let mut max:f32;

        for i in 0..self.cols{
            max = *self.get_col(i).iter().map( |&x| OrderedFloat(x))
                                         .max()
                                         .unwrap_or(OrderedFloat(std::f32::NAN));
            maxs.push(max)
        }

        return maxs;
    }
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

fn sensors_analysis(sensors_vec : &Vec<SensorData>, buffer_size : u32) -> (){
    //matrice capace di salvare 10 catture per 10 sensori (10x10)
    let mut sensors_matrix = SensorMatrix::new(buffer_size,10);
    let mut row = 0;

    for sens_data in sensors_vec{
        sensors_matrix.set_row(row, sens_data.values.to_vec()) ;
        row += 1;
    }

    /*row = 0;

     for sens_data in sensors_vec{
        col=0;
        for data in sens_data.values{
            sensors_matrix.set(row, col, data);
            col+=1;
       }
       row+=1;
    }*/

    let sensors_means = sensors_matrix.means();
    let sensors_min = sensors_matrix.min();
    let sensors_max = sensors_matrix.max();

    println!("CONSUMER : Medie : {:?}, Minimi : {:?}, Massimi : {:?}",sensors_means,sensors_min,sensors_max);

}

fn consume(data_file:&mut File) -> Result<(), fslock::Error>{
    let mut sensordata: SensorData = SensorData::default();
    let mut header : Header = Header::default();
    let size_of_head = size_of_val(&header);
    let mut sensor_vec:Vec<SensorData> = Vec::new();
   
    //buffer su misura per Header così da poter scrivere e leggere correttamente l'header ad ogni chiamata del producere e consumer
    let mut header_buf:Vec<u8> = vec![0;size_of_head+1];
    
    //lettura header per estrapolare stato corrente del buffer
    header = read_from_file(data_file, 0, &mut header_buf);

    let write_index = header.write_index;
    let buffer_size=header.buffer_size;
    let mut read_index = header.read_index;

    println!("CONSUMER avvio funzione consume(): write_index : {:?}, buffer_size : {:?}, read_index : {:?}",write_index,buffer_size,read_index);

    
    //rendo binario la struttura SensorData solo per calcolarne la dimensione per l'offset
    let mut sensor_buf = bincode::serialize(&sensordata).unwrap(); //questo serve solo per calcolare valore offset
    let mut offset :u32;


    //leggi i dati dei sensori

    for i in 0..10{

        //verifica che il buffer non sia vuoto (write_index =0  and read_index=0) e nel caso in cui lo fosse, attendi producer
        //elimina anche la possibilità di leggere l'ultimo valore del buffer se write_index>buffer_size e read_index>buffer_size
        if write_index > buffer_size &&  read_index > buffer_size{
            println!("CONSUMER : Buffer vuoto, attendi producer ");
            header.set_read_index(read_index);
            write_on_file(data_file, 0, &header);
            return Ok(());
        } else if read_index > buffer_size && write_index < buffer_size{
            println!("CONSUMER : Coda del buffer, ma possibile leggere ricominciando da 0");
            read_index = read_index % buffer_size;
        }else if read_index == 0 && write_index == 0{
            println!("CONSUMER : Buffer vuoto, attendi producer ");
            header.set_read_index(read_index);
            write_on_file(data_file, 0, &header);
            return Ok(());
        }else{
            println!("CONSUMER: Scrittura su {:?} e indice lettura su {:?}",write_index,read_index);
        }

        //lettura dati dal buffer
        offset = mem::size_of_val(&header_buf) as u32 + (mem::size_of_val(&sensor_buf) as u32)*(read_index);
        sensordata = read_from_file(data_file, offset as u64,&mut sensor_buf );
        println!("CONSUMER : Lettura buffer_circolare al read_index {:?} : sensordata -> {:?} ",read_index,&sensordata);
        sensor_vec.push(sensordata);
    

        read_index += i;
    

    }

    //aggiorna puntatore per la lettura (che intanto è stato aggiornato )
    header.set_read_index(read_index);

    //scrivi in binario la struttura Header data nel file come INTESTAZIONE INIZIALE (offset=0)
    write_on_file(data_file, 0, &header);

    sensors_analysis(&sensor_vec,buffer_size);

    println!("CONSUMER: Fine funzione consume() -> write_index : {:?}, buffer_size : {:?}, read_index : {:?}",write_index,buffer_size,read_index);
    
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
        
        result = consume(&mut data_file);
        
        lock_file.unlock()?;
        
        //sleep di 1 secondo
        let ten_sec = time::Duration::from_millis(10000);
        thread::sleep(ten_sec);

        result?
    }

}