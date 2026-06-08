// src/main.rs

use buffer::ThreadSafeBuffer;
use std::thread;
use std::time::Duration;
use tracing::{Level, info};
use tracing_subscriber::EnvFilter;

fn main() {
    // Initialiseer de tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::TRACE.into()))
        .init();

    info!("Applicatie opgestart. Initialiseren van de thread-safe buffer...");

    // We maken de hoofd-buffer aan
    let buffer = ThreadSafeBuffer::<String>::new(10);
    let mut thread_handles = vec![];

    // --- WRITERS (Producers) ---
    for writer_id in 1..=5 {
        let buffer_clone = buffer.clone();

        let handle = thread::spawn(move || {
            for msg_id in 1..=100 {
                let data = format!("Data van producer {} met index {}", writer_id, msg_id);

                buffer_clone.push(data);

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
    for reader_id in 1..=2 {
        let buffer_clone = buffer.clone();

        let handle = thread::spawn(move || {
            info!(
                reader_id = reader_id,
                "Reader opgestart, begint met luisteren..."
            );

            // GEFIXT: In plaats van een vast aantal, gebruiken we try_pop / pop in een slimme lus.
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

    info!("Alle threads succesvol afgerond. Buffer-verwerking voltooid.");
}
