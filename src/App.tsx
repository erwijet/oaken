import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/rpc";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/lib/ui/table";
import { Combobox } from "./lib/components/Combobox";
import { useState } from "react";
import { Button } from "@/lib/ui/button";
import { maybe, isNone, isSome } from "@tsly/maybe";
import { LedgerEntry, Team } from "./bindings";
import { useEventHandler } from "./lib/events";

function App() {
  const { data: teams, refetch } = useQuery({
    queryKey: [],
    queryFn: () => api.query(["getTeams"]),
  });

  useEventHandler("config_did_load", () => refetch());

  console.log({ data: teams ?? "unloaded" });

  const [team1id, setTeam1id] = useState<number | undefined>();
  const [team2id, setTeam2id] = useState<number | undefined>();

  const [ledgerEnt, setLedgerEnt] = useState<LedgerEntry | undefined>();

  return (
    <div className="p-4 w-full h-full flex flex-col gap-4">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Team</TableHead>
            <TableHead className="text-right">Skill Lvl</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {teams?.map((team) => (
            <TableRow>
              <TableCell>{team.name}</TableCell>
              <TableCell className="text-right">{team.skill}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>

      <div className="flex gap-4 items-center">
        <Combobox
          disabled={isSome(ledgerEnt)}
          items={teams ?? ([] as Team[])}
          value={teams?.find((it) => it.id == team1id)}
          onChange={(it) => setTeam1id(it?.id)}
          renderItemFn={(team) => team.name}
          intoQueryable={(team) => team?.name ?? ""}
          equalityFn={(a, b) => a.id == b.id}
        />
        VS
        <Combobox
          disabled={isSome(ledgerEnt)}
          items={teams ?? []}
          value={teams?.find((it) => it.id == team2id)}
          onChange={(it) => setTeam2id(it?.id)}
          renderItemFn={(team) => team.name}
          equalityFn={(a, b) => a.id == b.id}
          intoQueryable={(team) => team.name}
        />
        <Button
          disabled={isNone(team1id) || isNone(team2id)}
          onClick={() => {
            api
              .mutation([
                "simulate",
                {
                  id: 0,
                  wk_no: 0,
                  season_id: 0,
                  home_team_id: team1id!,
                  away_team_id: team2id!,
                },
              ])
              .then((ent) => setLedgerEnt(ent));
          }}
        >
          Simulate
        </Button>
        <Button
          variant={"secondary"}
          onClick={() => {
            setTeam1id(undefined);
            setTeam2id(undefined);
            setLedgerEnt(undefined);
          }}
        >
          Reset
        </Button>
      </div>
      <hr />
      {maybe(ledgerEnt)?.take((it) => (
        <div className="flex gap-4">
          <h4>Result: </h4>
          <div>
            {teams?.find((team) => team.id == team1id)?.name} (home) score: {it.home_score}
          </div>
          <div>
            {teams?.find((team) => team.id == team2id)?.name} score: {it.away_score}
          </div>
          <div>
            Winner:{" "}
            {it.home_score == it.away_score
              ? "Draw"
              : teams?.find((team) => team.id == (it.home_score > it.away_score ? team1id : team2id))?.name}
          </div>
        </div>
      ))}
    </div>
  );
}

export default App;
