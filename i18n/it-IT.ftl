scopa-app =
    .title-text = Segnapunti Scopa
    .author-text = Scritto da Nigel Eke; 2025-2026.
    .license-text = Licenza BSD-3-Clause.
    .version-text = v{$version}.

target-input =
    .label = Obiettivo
    .placeholder = Inserisci obiettivo
    .aria-label = Inserisci obiettivo

add-team-button =
    .hint = Aggiungi squadra
    .aria-label = Aggiungi squadra

remove-team-button =
    .hint = Rimuovi squadra
    .aria-label = Rimuovi squadra { $team }

team-name-input =
    .placeholder = Inserisci nome squadra
    .aria-label = Inserisci nome squadra

start-button =
    .text = Inizia
    .aria-label = Inizia punteggio

help-button =
    .title = Aiuto
    .aria-label = Regole del gioco e istruzioni

home-button =
    .title = Indietro
    .aria-label = Torna alla pagina principale


scopa-button =
    .aria-label =
        { $n ->
            [0] Nessuna scopa
            [1] Una scopa
           *[other] { $n } scope
        }

more-button =
    .text = Altro…
    .aria-label = Mostra altre opzioni

cancel-button =
    .text = Annulla
    .aria-label = Torna all’editor dei punti

undo-button =
    .hint = Annulla
    .aria-label = Annulla punteggio

score-button =
    .text = Segna
    .aria-label = Registra il punteggio corrente

scopa-icon =
    .hint = Scopa
    .alt-text = Icona della scopa

cards-count-icon =
    .hint = Conteggio delle carte
    .alt-text = Icona conteggio carte

coins-count-icon =
    .hint = Le monete contano
    .alt-text = Icona conteggio monete

settebello-icon =
    .hint = Settebello
    .alt-text = Icona Settebello

premiera-icon =
    .hint = Premiera
    .alt-text = Icona di Premiera

score-scopa-editor =
    .aria-label = Punteggio Scopa per {$teamname}

score-group-icon =
    .aria-label = {$group} per {$teamname}

round-view =
    .text = Giri {$n}

restart-button =
    .text = Ricomincia
    .aria-label = Ricomincia da capo

winner-view =
    .text = Vincitore - {$teamname} !!!

restart-settings =
    .text = Stesse squadre
    .aria-label = Seleziona per usare le stesse squadre

menu-icon =
    .alt-text = Tendina
restart-icon =
    .alt-text = Riprendere
help-icon =
    .alt-text = Aiuto
home-icon =
    .alt-text = Home
undo-icon =
    .alt-text = Annulla
fullscreen-icon =
    .alt-text = Schermo intero

lang =
    .en-GB = Inglese
    .it-IT = Italiano

reset-button =
    .text = Reset

# Help page
help =
    .heading = Benvenuto
    .intro =
        Benvenuti a Segnapunti Scopa, il compagno perfetto per le vostre
        serate di gioco a Scopa! Con questa utile utility potete tenere
        traccia dei punteggi, gestire i round e mantenere il divertimento
        a ritmo serrato.
    .rules-heading = Regole
    .rules-teams =
        Per 2, 3, 4 o 6 giocatori. Quattro giocatori possono giocare in due
        squadre da due o individualmente. Sei giocatori possono giocare in
        tre squadre da due, due squadre da tre o individualmente.
    .rules-deal =
        Distribuisci tre carte a ciascun giocatore. Distribuisci quattro carte
        al centro del tavolo, scoperte.
    .rules-aim =
        Durante il gioco gli obiettivi principali sono a) raccogliere il maggior
        numero di carte possibile, b) raccogliere il maggior numero di carte moneta
        possibile, c) raccogliere il sette di denari (Settebello) e d) raccogliere
        tutte le carte presenti sul tavolo in una singola giocata (Scopa).
    .rules-play-1 = Disponi una carta
    .rules-play-2 =
        Se il valore nominale corrisponde a quello di un'altra carta singola sul
        tavolo, prendile entrambe e mettile da parte per il conteggio successivo.
    .rules-play-3 =
        Se il valore nominale non corrisponde a una singola carta, puoi abbinarne
        più di una. Ad esempio, se sul tavolo ci sono 7, 5, 4 e 3 e giochi un
        altro 7, devi prendere il 7 sul tavolo e non il 4 e il 3. Tuttavia, se
        giochi un 8, puoi prendere il 5 e il 3. Nota: non sei obbligato a giocare
        il 7, ma se lo fai, devi prendere anche l'altro 7.
    .rules-play-4 =
        Se il valore nominale della carta che giochi è il totale di tutte le altre
        carte, allora raccoglile per una Scopa e annota il fatto che hai ricevuto
        anche una Scopa per il successivo conteggio. È normale raccogliere le carte
        in un mazzo coperto, ma annota ogni Scopa con una singola carta scoperta.
    .rules-play-5 =
        Se il giocatore precedente ha ricevuto una Scopa, il giocatore successivo
        deve semplicemente giocare una carta. Non è possibile distribuire altre
        quattro carte e il giocatore successivo non ha alcuna possibilità di
        riscuotere.
    .rules-play-6 =
        Una volta completate tutte le giocate, le carte sul tavolo vengono assegnate
        all'ultimo giocatore che ha raccolto le carte. Questo non conta come una Scopa.
    .starting-heading = Imposta squadre e punti obiettivo
    .starting-intro =
        Prima che inizi il conteggio dei punti, la schermata iniziale consente
        di impostare l'obiettivo per una partita vincente e di aggiungere i nomi
        delle squadre o dei giocatori che partecipano.
    .starting-points =
        Clicca sul target per modificare l'obiettivo.
    .starting-add-team =
        Inserisci il nome di una squadra, quindi clicca sul pulsante "+" per aggiungerla
        all'elenco dei partecipanti. Se giochi in coppia, aggiungi solo una squadra, ad
        esempio "Abbot & Costello".
    .starting-remove-team =
        Clicca sul '-' per rimuovere le squadre prima che inizi il punteggio.
    .starting-start-game =
        Seleziona [inizio] per iniziare a segnare il primo round del gioco.
    .scoring-heading = Giri di punteggio
    .scoring-intro =
        La schermata di punteggio permette di segnare ogni giro, fino a quando una persona
        o una squadra viene dichiarata vincitrice.
    .scoring-scopa =
        Inserisci il numero di scopa fatte da ciascuna squadra nella loro casella di conteggio
        delle scopate. Questo conteggio si riferisce solo al giro corrente che viene segnato.
    .scoring-basics =
        Per conteggiare le "carte", "monete", "settebello" e "premiere", seleziona l'icona
        sotto la squadra.
        L'icona "Nessuno" (l'icona più a sinistra) può essere selezionata per "carte", "monete"
        e "premiere", ma per il "settebello" deve essere selezionata una squadra.
    .scoring-undo =
        Dopo il primo turno, è possibile annullare un punteggio o fare il rollback se vengono
        commessi errori. Dopo il rollback, tutti i giri devono essere riapplicati manualmente.
    .home = Home
