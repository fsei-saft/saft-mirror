# Skripten- Abschlussarbeiten- und FinanzTool

Wilkommen zu SAFT!

SAFT ist ein Webservice Programm das mit der allgemeinen Tätigkeit der FSEI hilft. Das Ziel ist die Buchhaltung und Verkauf einheitlich zu machen und so bereit für die Zukunft!

[TOC]

## Aufbau

Alle Funktionen sind in Module aufgeteilt welche sich in den *Modules* Ordnern befinden. Jedes Modul hat ein Cargo-Manifest das das Modul beschreibt. Dependencies, Version, Autoren,… alles steht da drin.

Jedes Modul hat einen *main.rs*; also ist ein standalone Server. Es gibt 2 libraries: libsaft und include_jason.

Ein Cargo-Manifest Template sieht so aus:
```
[package]
name = "opfel"
version = "0.0.1"
authors = ["Linorym <linorym@fs.ei.tum.de>"]
edition = "2021"
description = "Opfel - das Skriptemanagement tool"

[dependencies]
rocket = { version = "0.5"}
rocket_okapi = { version = "0.8", features = ["rapidoc"] }

[features]
default = []
```

Zu beachten ist *rocket_okapi*. Ein Packet das es vereinfacht Dokumentation über API-Schnittstellen zu führen. Statt ```routes![]``` wird ```openapi_get_routes![]``` verwendet. Ein weiterer unterschied ist über ```#[get("/...")]``` ein ```#[openapi]``` gesetzt wird.

Und bei der Rocket Initialisierung wird noch rapidoc hinzugefügt
```.mount("/docs", make_rapidoc(&RapiDocConfig {
        general: GeneralConfig {
        spec_urls: vec![UrlObject::new("Resource", "/openapi.json")],
        ..Default::default()
        },
        ..Default::default()
        }))
```

## Libraries

Libraries haben keine ausführbare Dateien, sind aber esenziell zur Funktionsfähigkeit von SAFT.
- libsaft hat code und Dateien die allgemein zu benutzen sind um Einheitlichkeit zu behalten
- include_jason macht etwas was ich garnicht verstehe (Oli)

## Lizenz

Siehe [LICENSE.md](LICENSE.md).
