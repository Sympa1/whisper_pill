---
name: database-migration
description: Verwaltet versionierte Datenbankschemata, generiert sichere Migrationen mit SQLAlchemy/Alembic oder SQLite und erzwingt Backups sowie Rollback-Fähigkeit.
---

# SKILL: Datenbank-Migration & Sichere Schema-Verwaltung (`database-migration`)

> **Rolle & Einsatzzweck:**
> Dieser Skill leitet den **`researcher`-** und **`coder`-Agenten** bei Änderungen an Datenbankmodellen (`SQLite`, `PostgreSQL`, `SQLAlchemy`) an. Er stellt sicher, dass bestehende Daten niemals unkontrolliert verändert oder gelöscht werden.

---

## 1. Harte Sicherheits-Leitplanken (Core Harness)

1. **Keine unautorisierten Schema-Eingriffe:** Produktive oder lokale Bestandsdatenbanken dürfen niemals ohne expliziten Migrationspfad überschrieben oder mittels `DROP TABLE` gelöscht werden.
2. **Pflicht-Backup vor Migration:** Vor dem Anwenden einer Migration auf eine bestehende Datenbankdatei (z. B. `app.db`) muss zwingend eine Sicherheitskopie angelegt werden.
3. **Vollständige Rollback-Fähigkeit:** Jede Migration muss sowohl eine `upgrade()`- als auch eine funktionierende `downgrade()`-Logik besitzen.
4. **Parametrisierte Abfragen:** Auch innerhalb von Daten-Migrationen sind ausschließlich gebundene Parameter (`:param` / `?`) erlaubt (SQL-Injection-Schutz).

---

## 2. Standard-Befehle (Alembic & SQLAlchemy)

```bash
# A) Backup einer lokalen SQLite-Datenbank vor der Migration erstellen:
cp data/app.db data/app.db.backup

# B) Neue Migration aus Model-Änderungen generieren:
alembic revision --autogenerate -m "Beschreibung der Migration"

# C) Generierte Migrationsdatei IMMER manuell mit view_file prüfen!

# D) Migration auf den aktuellen Stand heben:
alembic upgrade head

# E) Rollback der letzten Migration testen:
alembic downgrade -1
```

---

## 3. Checkliste für Schema-Änderungen

- `[ ]` **Abwärtskompatibilität:** Neue Spalten auf bestehenden Tabellen besitzen entweder ein sinnvolles `DEFAULT` oder sind `NULL`-fähig (`nullable=True`), damit bestehende Datensätze gültig bleiben.
- `[ ]` **Zweistufiges Löschen:** Spalten oder Tabellen werden nicht sofort gelöscht, sondern im ersten Schritt als `deprecated` entkoppelt.
- `[ ]` **In-Memory-Migrations-Test:** Der `tester`-Agent prüft Upgrade und Downgrade der Migration automatisiert gegen eine isolierte Test-Datenbank (`sqlite:///:memory:`).
