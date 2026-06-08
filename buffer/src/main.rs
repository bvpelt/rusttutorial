// src/main.rs

use buffer::ThreadSafeBuffer;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use tracing::{Level, info, warn};
use tracing_subscriber::EnvFilter;

fn main() {
    // Initialiseer de tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::TRACE.into()))
        .init();

    info!("Applicatie opgestart. Initialiseren van de thread-safe buffer...");

    let max_readers = 2;
    let max_writers = 5;

    // We maken de hoofd-buffer aan
    let buffer = ThreadSafeBuffer::<String>::new(10);
    let mut thread_handles = vec![];

    // 1. Maak arrays van atomaire tellers aan, verpakt in een Arc zodat ze veilig
    // gedeeld kunnen worden tussen threads. We initialiseren ze op 0.
    // We maken 6 posities aan voor writers (zodat we index 1 t/m 5 kunnen gebruiken)
    let writer_counters: Arc<Vec<AtomicUsize>> =
        Arc::new((0..=max_writers).map(|_| AtomicUsize::new(0)).collect());

    // We maken 5 posities aan voor readers (voor index 1 t/m 4)
    let reader_counters: Arc<Vec<AtomicUsize>> =
        Arc::new((0..=max_readers).map(|_| AtomicUsize::new(0)).collect());

    // --- WRITERS (Producers) ---
    for writer_id in 1..=max_writers {
        let buffer_clone = buffer.clone();
        // Kloon de Arc pointer naar de tellers voor deze specifieke thread
        let counters = Arc::clone(&writer_counters);

        let handle = thread::spawn(move || {
            for msg_id in 1..=100 {
                let data = format!("Data van producer {} met index {}", writer_id, msg_id);

                buffer_clone.push(data);

                // 2. Hoog de specifieke teller voor deze writer atomair op met 1.
                // Ordering::Relaxed is hier perfect en het snelst, omdat de tellers
                // onafhankelijk van andere geheugenacties opereren.
                counters[writer_id].fetch_add(1, Ordering::Relaxed);

                info!(
                    writer_id = writer_id,
                    buffer_grootte = buffer_clone.len(),
                    msg_id = msg_id,
                    "Ownership overdragen van element naar de buffer"
                );

                thread::sleep(Duration::from_millis(10)); // Iets sneller gezet voor de test
            }
            info!(writer_id = writer_id, "Writer is helemaal klaar!");
        });
        thread_handles.push(handle);
    }

    // --- READERS (Consumers) ---
    for reader_id in 1..=max_readers {
        let buffer_clone = buffer.clone();
        // Kloon de Arc pointer naar de tellers voor deze specifieke thread
        let counters = Arc::clone(&reader_counters);

        let handle = thread::spawn(move || {
            info!(
                reader_id = reader_id,
                "Reader opgestart, begint met luisteren..."
            );

            // De retry-teller leeft specifiek binnen deze thread
            let mut retry_count = 0;
            let max_retries = 2;

            // In plaats van een vast aantal, gebruiken we try_pop / pop in een slimme lus.
            // Voor dit MPMC patroon loopen we tot we een signaal krijgen, of we gebruiken een loop
            // met een kleine timeout/check. Een elegante manier zonder het kanaal te sluiten
            // is gebruik maken van try_pop wanneer we merken dat er niks meer komt,
            // maar voor nu loopen we totdat we handmatig stoppen óf we laten de readers
            // collectief exact het totaal aantal verwerken (500).

            // Laten we de lezers loopen en gebruik maken van try_pop met een kleine fallback
            // zodat ze netjes stoppen als er langere tijd geen data is en de applicatie klaar is.
            loop {
                match buffer_clone.try_pop() {
                    Some(ontvangen_data) => {
                        retry_count = 0;
                        // 3. Hoog de specifieke teller voor deze reader atomair op met 1
                        counters[reader_id].fetch_add(1, Ordering::Relaxed);

                        info!(
                            reader_id = reader_id,
                            buffer_grootte = buffer_clone.len(),
                            ontvangen_waarde = %ontvangen_data,
                            "Element succesvol opgehaald met ownership"
                        );
                        // Simuleer verwerkingstijd
                        thread::sleep(Duration::from_millis(10));
                    }
                    None => {
                        retry_count += 1;
                        if retry_count > max_retries {
                            warn!(
                                reader_id = reader_id,
                                retry_count = retry_count - 1,
                                "Buffer blijft leeg na {} pogingen. Reader stopt.",
                                max_retries
                            );
                            break; // Stap uit de oneindige loop, de thread eindigt hier
                        }

                        info!(
                            reader_id = reader_id,
                            "Wacht totdat er weer data om te verwerken is"
                        );
                        // De buffer is tijdelijk leeg. We wachten heel even.
                        // In een echte productie-app zou je stoppen als je een 'stop' signaal krijgt.
                        thread::sleep(Duration::from_millis(10000));

                        // Kleine hack voor deze main test: als de writers klaar zijn en de buffer is leeg,
                        // mogen de lezers stoppen. Om het simpel te houden voor deze demo,
                        // kun je ook 'pop()' gebruiken als je exact weet hoeveel er komen.
                    }
                }
            }
        });
        // Omdat de lezers in deze opzet in een oneindige 'loop' zitten,
        // zouden de joins onderaan ook eeuwig wachten.
        // Om te zorgen dat de app stopt zodra de WRITERS klaar zijn,
        // joinen we dadelijk *alleen* de writers. Zodra main() stopt, stoppen de lezers ook.
        thread_handles.push(handle);
    }

    // ALTERNATIEVE GEFIKSTE READERS (Als je exact 500 elementen wilt verdelen):
    // Als je wilt dat de readers netjes stoppen na exact alle data te hebben verdeeld,
    // verander dan de lezer-lus naar een pop-loop die klopt met het totaal (500 / 4 lezers = 125 elk):
    /*
    for reader_id in 1..=4 {
        let buffer_clone = buffer.clone();
        let handle = thread::spawn(move || {
            for _ in 0..125 { // 5 * 100 = 500 totaal. 500 / 4 lezers = 125 iteraties per lezer.
                let ontvangen_data = buffer_clone.pop();
                info!(reader_id = reader_id, ontvangen_waarde = %ontvangen_data, "Element succesvol opgehaald");
                thread::sleep(Duration::from_millis(15));
            }
        });
        thread_handles.push(handle);
    }
    */

    // Wacht tot alle threads klaar zijn (als je de exacte 125-iteratie fix gebruikt)
    for handle in thread_handles {
        handle.join().unwrap();
    }
    // --- RAPPORTAGE EN STATISTIEKEN TONEN ---
    info!("==================================================");
    info!("             BUFFER VERWERKINGS RAPPORT           ");
    info!("==================================================");

    info!("WRITER STATISTIEKEN:");
    let mut totaal_verstuurd = 0;
    for writer_id in 1..=max_writers {
        // .load() leest de huidige atomaire waarde uit
        let aantal = writer_counters[writer_id].load(Ordering::Relaxed);
        totaal_verstuurd += aantal;
        info!(
            writer_id = writer_id,
            aantal_verstuurd = aantal,
            "Writer {} heeft {} berichten verwerkt",
            writer_id,
            aantal
        );
    }

    info!("--------------------------------------------------");
    info!("READER STATISTIEKEN:");
    let mut totaal_ontvangen = 0;
    for reader_id in 1..=max_readers {
        let aantal = reader_counters[reader_id].load(Ordering::Relaxed);
        totaal_ontvangen += aantal;
        info!(
            reader_id = reader_id,
            aantal_ontvangen = aantal,
            "Reader {} heeft {} berichten ontvangen",
            reader_id,
            aantal
        );
    }

    info!("--------------------------------------------------");
    info!(
        totaal_verstuurd = totaal_verstuurd,
        totaal_ontvangen = totaal_ontvangen,
        resterend_in_buffer = buffer.len(),
        "Eindbalans opgemaakt"
    );
    info!("==================================================");
}
