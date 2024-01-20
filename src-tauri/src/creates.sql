-- master table
CREATE TABLE ctrl (
    schema_ver INTEGER NOT NULL,
    year INTEGER NOT NULL,
    wk_no INTEGER NOT NULL
);

CREATE TABLE leagues (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    abbr TEXT NOT NULL 
);

CREATE TABLE tiers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    rank INTEGER NOT NULL,
    name TEXT NOT NULL,
    league_id INTEGER NOT NULL,

    FOREIGN KEY (league_id) REFERENCES leagues (id),
    CONSTRAINT unique_name_per_league UNIQUE (name, league_id)
);

CREATE TABLE teams (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    skill INTEGER NOT NULL,
    league_id INTEGER NOT NULL,
    tier_id INTEGER NOT NULL,

    FOREIGN KEY (tier_id) REFERENCES tiers (id)
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
    tier_id INTEGER NOT NULL,
    league_id INTEGER NOT NULL,
    year INTEGER NOT NULL,

    CONSTRAINT unique_year_per_tier UNIQUE (year, tier_id)
);

--

INSERT INTO ctrl (schema_ver, year, wk_no) VALUES ($1, 2023, 1);