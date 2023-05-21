## sequenza di operazioni elementari (somma,sottrazione, moltiplicazione e divisione) necessarie per ottenere 10, partendo da 5 numeri da a 9 casuali

# Vincoli :
        - le cinque cifre sono comprese tra 0 e 9 e possono ripetersi
        - le cifre devono essere utilizzate tutte, in qualsiasi ordine
        - non ci sono limiti sulle operazioni (es. vanno bene anche quattro somme)
        - non si considera la precedenza degli operatori, le operazioni vanno applicate da sinistra a destra secondo il loro ordine
    Esempio: dato 2 7 2 2 1 una soluzione può essere 7 - 2 - 1 x 2 + 2 = 10

# Cosa fare :
    1) Leggi la sequenda di numeri da LINEA DI COMANDO 
    2) Trova tutte le possibili soluzioni e salvale in una collezione di stringhe (es: “7 - 2 - 1 x 2 + 2” )

# Suggerimenti :
    1) Per risolverlo utilizzare un approccio brute force:
        Elencare in un vettore tutte le possibili permutazioni delle cinque cifre 
            Assieme ogni permutazione di cifre, tutte le possibili permutazioni di quattro operazioni elementari e calcola risultato per ciascuna di esse
            (Se il risultato è 10 la permutazione viene salvata, altrimenti viene scartata)
    2) Essendo cifre e simboli delle operazioni di tipo differente utilizzare un vettore di tuple: il primo elemento sono le 5 cifre permutate, il secondo le operazioni (credo a questo punto che se sono possibili più combinazioni di operazioni ripeti la permutazione di cifre in un'altra tupla con la nuova combinazione di operazioni).       

# THREAD: sfruttiamo i thread per velocizzare la ricerca delle soluzioni in parallelo

    1) Si divide il vettore di tutte le possibili permutazioni in n blocchi uguali e per ciascuna si lancia un thread. Provare con n=2,3,4… ecc, 2) misurare i tempi 
    3) trovare il numero di thread oltre il quale non vi sono vantaggi

    Cambia qualcosa se la divisione del lavoro fra thread anziché essere a blocchi è interleaved? Vale a dire con tre thread il primo prova le permutazioni con indice 0,3,6,... il secondo 1,4,7,... e il terzo 2,5,8,...