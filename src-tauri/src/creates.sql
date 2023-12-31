-- master table
CREATE TABLE ctrl (
    schema_ver INTEGER NOT NULL,
    year INTEGER NOT NULL,
    wk_no INTEGER NOT NULL
);

-- teams table
CREATE TABLE teams (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    skill INTEGER NOT NULL
);

-- matches table
CREATE TABLE matchups (
    id INTEGER PRIMARY KEY,
    wk_no INTEGER NOT NULL,
    season_id INTEGER NOT NULL,
    home_team_id INTEGER NOT NULL,
    away_team_id INTEGER NOT NULL,
    home_team_score INTEGER,
    away_team_score INTEGER,
    schedule_id INTEGER,

    FOREIGN KEY (home_team_id) REFERENCES teams (id),
    FOREIGN KEY (away_team_id) REFERENCES teams (id),
    FOREIGN KEY (schedule_id) REFERENCES schedules (id)
);

-- schedules table
CREATE TABLE schedules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    year INTEGER NOT NULL UNIQUE
);

--

INSERT INTO ctrl (schema_ver, year, wk_no) VALUES ($1, 2023, 1);