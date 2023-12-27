-- teams table
CREATE TABLE IF NOT EXISTS teams (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    skill INTEGER NOT NULL
);

-- matches table
CREATE TABLE IF NOT EXISTS matches (
    id INTEGER PRIMARY KEY,
    wk_no INTEGER NOT NULL,
    season_id INTEGER NOT NULL,
    home_team_id INTEGER NOT NULL,
    away_team_id INTEGER NOT NULL,

    FOREIGN KEY (home_team_id) REFERENCES teams (id),
    FOREIGN KEY (away_team_id) REFERENCES teams (id)
);

-- ledger_entries table
CREATE TABLE IF NOT EXISTS ledger_entries (
    match_id INTEGER NOT NULL,
    home_score INTEGER NOT NULL,
    away_score INTEGER NOT NULL,

    FOREIGN KEY (match_id) REFERENCES matches (id)
);
