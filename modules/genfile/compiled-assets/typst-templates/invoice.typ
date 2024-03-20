#import "tablex.typ": tablex
#import "letter-pro.typ": letter-generic

#let data = json("data.json")

#letter-generic(
    margin: (
        bottom: 9em
    ),
    header: [
        #pad(x: 2cm, top: 1cm)[
            #grid(
                columns: (3fr, 2fr),
                rows: (auto),
                [
                    #text(size: 26pt)[Fachschaft Elektrotechnik und Informationstechnik e.V.] \
                    #text(size: 16pt)[an der Technischen Universität München]
                ],
                [#align(right + top)[#image("trafo-bw.svg", height: 4cm)]]
            )
        ]
    ],
    footer: [
        #set text(size: 9pt)
        #grid(
            columns: (6fr, 4fr, 3fr),
            rows: (14pt),
            [*Briefanschrift*: Postfach, 80290 München],
            [*Telefon*: (089) 289 - 22998],
            [*E-Mail*: info\@fs.ei.tum.de],
            [*Lieferanschrift*: Theresienstraße 90, 80333 München],
            [*Telefon Büro*: (089) 289 - 22960],
            []
        )
        #v(0pt, weak: true)
        #grid(
            columns: (10fr, 3fr),
            rows: (14pt),
            [*Bankverbindung*: Stadtsparkasse München, IBAN: DE97 7015 0000 0901 2321 16],
            [*USt-IdNr.*: DE129515443],
        )
    ],
    address-box: [
        #line(length: 100%)
        #v(0.5em, weak: true)
        #text(size: 9pt)[Fachschaft Elektrotechnik und Informationstechnik e. V.]
        #v(5pt, weak: true)
        #text(size: 9pt)[TU München, 80290 München]
        #v(0.5em, weak: true)
        #line(length: 100%)

        #data.recipient.address

        #align(bottom)[#line(length: 100%)]
    ]
)[
    #align(right)[München, #data.date]

    *Rechnung Nr. #data.number*

    #v(1em)

    Sehr geehrte Damen und Herren,

    vielen Dank für Ihren Auftrag. Wir erlauben uns Ihnen folgende Positionen in Rechnung zu stellen:

    #tablex(
        columns: (1fr, 2fr, 4fr, 2fr),
        rows: (14pt, auto),
        header-rows: 1,
        [#text(size: 7pt)[_Anzahl_]], [#text(size: 7pt)[_Art. Nr._]], [#text(size: 7pt)[_Beschreibung_]], [#text(size: 7pt)[_Gesamtpreis_]],
        ..data.items.map((item) => (
            [#item.quantity], [#item.number], [#item.description], [#item.total€],
        )).flatten()
    )

    #v(5pt, weak: true)

    #let items = data.items.map((item) => (
        [#item.quantity], [POST3415], [#item.description], [#item.total],
    )).flatten();

    #tablex(
        columns: (7fr, 2fr),
        rows: (auto),
        [Gesamtsumme], [*#data.total€*],
        ..data.taxes.map((tax) => (
            [#tax.description], [#tax.total€],
        )).flatten()
    )

    Die Leistung wurde am 07.03.2024 erbracht.

    Bitte überweisen Sie den fälligen Gesamtbetrag unter Angabe der Rechnungsnummer innerhalb von 14 Tagen auf unser unten angegebenes Konto.

    #v(1em)

    Mit freundlichen Grüßen

    #v(3em)

    #data.issuer
]
