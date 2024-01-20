// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Procedures = {
    queries: 
        { key: "getGameState", input: never, result: GameState } | 
        { key: "getMatchupsByWeek", input: GetMatchupsByWeekArgs, result: Matchup[] } | 
        { key: "getSchedule", input: number, result: Schedule[] } | 
        { key: "getSchedules", input: never, result: Schedule[] } | 
        { key: "getStandings", input: number, result: Standing[] } | 
        { key: "getTeamInfos", input: never, result: TeamInfo[] } | 
        { key: "getTeamMatchups", input: number, result: Matchup[] } | 
        { key: "getTeams", input: never, result: Team[] },
    mutations: 
        { key: "advanceWeek", input: never, result: null },
    subscriptions: never
};

export type GetMatchupsByWeekArgs = { year: number; wkNo: number }

export type Team = { id: number; name: string; skill: number; tier_id: number }

export type Matchup = { id: number; wkNo: number; homeTeamId: number; awayTeamId: number; homeTeamScore: number | null; awayTeamScore: number | null }

export type League = { id: number; name: string; abbr: string }

export type Standing = { teamId: number; teamName: string; wins: number; losses: number; draws: number; pointsFor: number; pointsAgainst: number; streak: number; winPercent: number }

export type Tier = { id: number; name: string; rank: number; leagueId: number }

export type GameState = { schema_ver: number; year: number; wk_no: number }

export type Schedule = { id: number; year: number; tier_id: number; league_id: number; matchups: Matchup[] }

export type TeamInfo = { id: number; name: string; skill: number; tier: Tier; league: League }
