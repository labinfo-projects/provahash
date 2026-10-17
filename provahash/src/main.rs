// Esempio d'uso di una HashMap in RUST
// In Rust, una HashMap è una struttura dati,  simile a un dizionario, 
// che associa una chiave (key) a un valore (value), permettendoti di 
// recuperare rapidamente un valore conoscendo la sua chiave.
use std::collections::HashMap;

fn main() {
    // Creiamo una HashMap che associa nomi (String) a età (u32).
    let mut eta: HashMap<String, u32> = HashMap::new();

    // Inserimento di coppie chiave-valore.
    eta.insert(String::from("Anna"), 28);
    eta.insert(String::from("Luca"), 35);
    eta.insert(String::from("Marta"), 31);
    eta.insert(String::from("Nane"), 81);
    eta.insert(String::from("Alvise"), 60);
    eta.insert(String::from("Tony"), 51);
    eta.insert(String::from("Franco"), 66);
    eta.insert(String::from("Gigi"), 73);
    // Se la chiave esiste già, insert sostituisce il valore precedente.
    eta.insert(String::from("Luca"), 36);

    // Ricerca: get restituisce un Option<&V>.
    // Se la chiave esiste, otteniamo Some(&valore); altrimenti None.
    match eta.get("Luca") {
        Some(anni) => println!("Luca ha {anni} anni."),
        None => println!("Luca non è presente."),
    }

    // Verifica se una chiave è presente.
    if eta.contains_key("Marta") {
        println!("Marta è nella mappa.");
    }

    // Inserisce un valore solo se la chiave non esiste.
    // entry evita di sovrascrivere un eventuale valore già presente.
    eta.entry(String::from("Bepi")).or_insert(64);

    // Aggiorna il valore associato a una chiave, se presente.
    if let Some(anni) = eta.get_mut("Marta") {
        *anni += 1;
    }

    // Rimozione: remove restituisce Option<V>.
    if let Some(anni) = eta.remove("Bepi") {
        println!("Bepi è stato espulso (aveva {anni} anni).");
    }
    if let Some(anni) = eta.remove("Marta") {
        println!("Marta è stata espulsa (aveva {anni} anni).");
    }
    // Iterazione: l'ordine non è garantito da HashMap.
    println!("\nPersone rimaste:");
    for (nome, anni) in &eta {
        println!("{nome}: {anni}");
    }

	// L'orinamento può essere realizzato raccogliend i riferimenti in un Vec ed elaborarlo.
	let mut persone: Vec<_> = eta.iter().collect();
	persone.sort_by_key(|(nome, _)| *nome);
	println!("\nPersone in ordine alfabetico:");
	for (nome, anni) in persone {
		println!("{nome}: {anni}");
	}
    // Esempio classico: contare quante volte compare ogni parola.
    let testo = "rosso blu rosso verde blu rosso";
    let mut conteggi: HashMap<&str, u32> = HashMap::new();

    for parola in testo.split_whitespace() {
        // Se la parola non esiste, la inserisce con valore 0;
        // poi incrementa il conteggio.
        *conteggi.entry(parola).or_insert(0) += 1;
    }

    println!("\nConteggio delle parole:");
    for (parola, numero) in &conteggi {
        println!("{parola}: {numero}");
    }
}