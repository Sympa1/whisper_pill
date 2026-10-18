---
name: tech-docs
description: Erstellt und pflegt die technische Entwickler-, API- und Architektur-Dokumentation in Markdown inklusive umfassender UML- und Architektur-Diagramme (Mermaid.js: Klassen-, Komponenten-, Sequenz-, Zustands- und ER-Diagramme) und führt docs/JOURNAL.md fort.
---

# SKILL: Technische Architektur-Dokumentation & Mermaid-UML (`tech-docs`)

> **Rolle & Einsatzzweck:**
> Dieser Skill standardisiert die Erstellung und Pflege der technischen Entwickler-Dokumentation (`docs/architecture/`, `docs/api/`, `docs/TECHNICAL_DOCS.md`) sowie des chronologischen Projekt-Journals (`docs/JOURNAL.md`) mit reichhaltigen, direkt in Markdown gerenderten Architektur- und UML-Diagrammen.

---

## 1. Ausgabeformat, Struktur & Datenschutz

- **Format:** Reines GitHub-Flavored Markdown (GFM) auf **Deutsch** (Code-Bezeichner, Klassennamen und Methoden bleiben auf **Englisch**).
- **Ablageort:** `docs/TECHNICAL_DOCS.md`, `docs/architecture/`, `docs/api/` sowie `docs/JOURNAL.md`.
- **Pflicht-Inhalte:**
  1. **System- & Schichtenarchitektur:** Überblick über Module, Packages und Verantwortlichkeiten nach dem Single Responsibility Principle (SRP).
  2. **Domänenmodell & Klassendesign:** Beschreibung der Kernklassen, Entitäten, Kapselung und Schnittstellen.
  3. **Datenflüsse & Laufzeitverhalten:** Abläufe wichtiger Geschäftsprozesse, Puffer-Konzepte (`sBuffy` für String-Puffer, `iBuffy` für Integer-/Zähler-Puffer), Zustandsautomaten (State Machines) und Thread-/Async-Architektur.
  4. **API- & Schnittstellen-Spezifikation:** Wichtigste Methoden-Signaturen, Parameter, Rückgabewerte und Fehlerbehandlung (Exceptions).
- **Strikter Datenschutz (Public-Repo-Konformität):**
  - Verwende in Dokumentationen **niemals absolute Dateipfade des Entwickler-Systems** (wie `/home/<username>/...` oder `C:\Users\<username>\...`).
  - Nutze ausschließlich sprechende Komponentenbezeichner (z. B. `UserService`, `OrderRepository`) oder relative Projektpfade (`src/...`, `tests/...`).

---

## 2. Verbindliche Architektur- & UML-Diagramme (Mermaid.js)

Jede technische Dokumentation **muss** die Architektur und Abläufe visuell durch passende `mermaid`-Diagramme veranschaulichen (ohne externe Bild-Abhängigkeiten). Wähle je nach Kontext mindestens **drei** der folgenden Diagrammtypen:

### A) System- & Schichtenarchitektur (`flowchart TD` / `LR`)
Visualisiert den modularen Aufbau, Schichten (Presentation, Service/Domain, Data) und Abhängigkeitsrichtungen:
```mermaid
flowchart TD
    subgraph Presentation["Presentation / CLI / API Layer"]
        Ctrl["Controller / Entrypoint"]
    end

    subgraph Domain["Domain & Service Layer (src/)"]
        Svc["UserService<br/>(Geschäftslogik & Validierung)"]
        Model["User<br/>(OOP-Entität & Kapselung)"]
    end

    subgraph Infra["Infrastructure / Storage"]
        Store[("In-Memory / DB Storage")]
    end

    Ctrl -->|"ruft auf"| Svc
    Svc -->|"instanziiert & mutiert"| Model
    Svc -->|"persistiert / lädt"| Store
```

### B) UML-Klassendiagramm (`classDiagram`)
Zeigt Klassen, Attribute, Methoden-Signaturen, Sichtbarkeiten (`+` public, `-` private) und Beziehungen (Komposition, Aggregation, Abhängigkeit):
```mermaid
classDiagram
    class User {
        +int user_id
        +str username
        +str email
        +bool is_active
        +deactivate() None
    }

    class UserService {
        -dict~int, User~ _users
        +register_user(user_id: int, username: str, email: str) User
        +find_user_by_id(user_id: int) User | None
        +deactivate_user(user_id: int) bool
    }

    UserService "1" *-- "0..*" User : verwaltet >
```

### C) Sequenzdiagramm für Kernprozesse (`sequenceDiagram`)
Dokumentiert das zeitliche Zusammenspiel der Objekte inklusive Puffer-Nutzung (`sBuffy` / `iBuffy`) und Fehlerpfaden (`alt` / `else`):
```mermaid
sequenceDiagram
    autonumber
    actor Caller as Aufrufer / Client
    participant Svc as UserService
    participant Entity as User (Model)

    Caller->>Svc: register_user(user_id, username, email)
    activate Svc
    alt E-Mail ungültig (kein '@')
        Svc-->>Caller: ValueError("Ungültige E-Mail-Adresse")
    else E-Mail valide
        Svc->>Entity: User(user_id, username, email)
        activate Entity
        Entity-->>Svc: user_instance (is_active=True)
        deactivate Entity
        Svc->>Svc: Speichere in _users[user_id] (Zähler iBuffy)
        Svc-->>Caller: user_instance
    end
    deactivate Svc
```

### D) Zustandsdiagramm / State Machine (`stateDiagram-v2`)
Modelliert den Lebenszyklus einer Entität, eines UI-Zustands oder eines Worker-Prozesses:
```mermaid
stateDiagram-v2
    [*] --> Unregistered: Systemstart
    Unregistered --> Active: register_user() [Valide Daten]
    Active -->Processing: Verarbeitung (sBuffy / iBuffy)
    Processing --> Active: Abschluss
    Active --> Inactive: deactivate_user() / user.deactivate()
    Inactive --> [*]
```

### E) Datenmodell / Entity-Relationship-Diagramm (`erDiagram`)
Bei Datenbank- oder Persistenz-Bezug zur Darstellung von Tabellen, Schlüsseln (`PK`, `FK`) und Kardinalitäten:
```mermaid
erDiagram
    USER ||--o{ AUDIT_LOG : "erzeugt"
    USER {
        int user_id PK
        string username
        string email
        boolean is_active
    }
    AUDIT_LOG {
        int log_id PK
        int user_id FK
        string action_summary
        datetime created_at
    }
```

> **Mermaid-Qualitätsregeln:**
> - Setze Knotenbeschriftungen mit Sonderzeichen (Klammern, Schrägstriche, Doppelpunkte) immer in doppelte Anführungszeichen: `Node["Text (Info)"]`.
> - Prüfe vor Abschluss, dass alle `mermaid`-Blöcke syntaktisch valide geschlossen sind.

---

## 3. Pflege des Projekt-Journals (`docs/JOURNAL.md`)

Jeder abgeschlossene Entwicklungsschritt oder Dokumentations-Meilenstein wird chronologisch **ganz oben** in `docs/JOURNAL.md` nach folgendem Schema ergänzt:

```markdown
## [YYYY-MM-DD HH:MM] – [Kurztitel des Meilensteins / Features]
- **Durchgeführte Änderungen:** [Präzise Beschreibung der umgesetzten Komponenten / Dokumentation]
- **Architekturentscheidungen (Warum):** [Begründung für gewählte Muster/Datenstrukturen]
- **Verifikation:** `[Unit-Tests PASSED | Security-Audit PASSED | Docs UPDATED]`
- **Offene Punkte / Nächste Schritte:** [To-Dos für die Folgesitzung]
```

