import { useQuery } from "@tanstack/react-query";
import { useNavigate, useParams } from "react-router";
import { api } from "./lib/rpc";
import { err } from "@tsly/core";
import { Badge } from "./lib/ui/badge";
import { Button } from "./lib/ui/button";
import { ArrowLeft, AtSign } from "lucide-react";
import { Table, TableBody, TableCell, TableFooter, TableHead, TableHeader, TableRow } from "./lib/ui/table";
import { maybe } from "@tsly/maybe";
import { iter } from "@tsly/iter";
import { Matchup, TeamInfo } from "./bindings";
import { TeamSelect } from "@/lib/prefab/TeamSelect";
import { NextWeekButton } from "@/lib/prefab/NextWeekButton";

function TeamPage() {
  const id: string = useParams()["id"] ?? err("missing :id");
  const nav = useNavigate();

  const { data: game } = useQuery({
    queryKey: ["getGameState"],
    queryFn: () => api.query(["getGameState"]),
  });

  const { data: matchups } = useQuery({
    queryKey: ["getTeamMatchups", id],
    queryFn: () => api.query(["getTeamMatchups", parseInt(id)]),
  });

  const { data: teams } = useQuery({
    queryKey: ["getTeamInfos"],
    queryFn: () => api.query(["getTeamInfos"]),
  });

  function getTeamById(id: number | string): TeamInfo | undefined {
    return teams?.find((it) => it.id == parseInt(id.toString()));
  }

  return (
    <div className="m-4">
      <div className="flex my-4 justify-between">
        <div className="flex gap-2 items-center">
          <Button variant={"ghost"} size={"icon"} onClick={() => nav(-1)}>
            <ArrowLeft />
          </Button>

          <div className="flex flex-col">
            <h1 className="text-4xl font-extrabold tracking-tight">{getTeamById(id)?.name}</h1>

            <div className="flex gap-2 items-center">
              <span>
                <Badge>
                  {getTeamById(id)?.league.name} // {getTeamById(id)?.tier.name}
                </Badge>
              </span>
              <span>
                <Badge variant={"secondary"}>Skill: {getTeamById(id)?.skill}</Badge>
              </span>
            </div>
          </div>
        </div>
        <div className="flex gap-2">
          <TeamSelect />
          <NextWeekButton />
        </div>
      </div>

      <hr />

      <h3 className="text-2xl text-gray-700 font-bold my-4 ml-8">Schedule ({game?.year})</h3>

      <div className="w-[calc(100%-4rem)] mx-8 border-px rounded-md shadow-lg">
        <Table>
          <TableHeader>
            <TableHead>Week No.</TableHead>
            <TableHead>Away Team</TableHead>
            <TableHead>Home Team</TableHead>
            <TableHead>Result</TableHead>
          </TableHeader>
          <TableBody>
            {matchups?.map((matchup) => (
              <TableRow>
                <TableCell>{matchup.wkNo}</TableCell>
                <TableCell>
                  {maybe(getTeamById(matchup.awayTeamId))?.take((team) => (
                    <Button variant={"link"} onClick={() => nav("/team/" + team.id)}>
                      {team.name}
                    </Button>
                  ))}
                </TableCell>
                <TableCell>
                  {maybe(getTeamById(matchup.homeTeamId))?.take((team) => (
                    <Button variant={"link"} onClick={() => nav("/team/" + team.id)}>
                      <AtSign className="w-4 h-4 mr-2" />
                      {team.name}
                    </Button>
                  ))}
                </TableCell>
                <TableCell>
                  {maybe(matchup.homeTeamScore)?.take((homeTeamScore) =>
                    maybe(matchup.awayTeamScore)?.take((awayTeamScore) => {
                      const scores = `${awayTeamScore} - ${homeTeamScore}`;

                      if (homeTeamScore > awayTeamScore && matchup.homeTeamId == parseInt(id)) return `Win (${scores})`;
                      if (homeTeamScore < awayTeamScore && matchup.awayTeamId == parseInt(id)) return `Win (${scores})`;
                      if (homeTeamScore == awayTeamScore) return `Draw (${scores})`;

                      return `Loss (${scores})`;
                    }),
                  ) ?? "-"}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
          <TableFooter>
            <TableRow>
              <TableCell colSpan={3}>Win/Loss</TableCell>
              <TableCell className="text-center">
                {iter(matchups ?? [])
                  .filterMap((match) => {
                    return (
                      maybe(match.homeTeamScore)?.take((homeTeamScore) =>
                        maybe(match.awayTeamScore)?.take((awayTeamScore) => {
                          if (homeTeamScore == awayTeamScore) return null; // skip
                          return [match, homeTeamScore, awayTeamScore] as readonly [match: Matchup, homeScore: number, awayScore: number];
                        }),
                      ) ?? null
                    );
                  })
                  .partition(([match, homeTeamScore, awayTeamScore]) => {
                    if (homeTeamScore > awayTeamScore && match.homeTeamId == parseInt(id)) return true; // win
                    if (homeTeamScore < awayTeamScore && match.awayTeamId == parseInt(id)) return true; // win

                    return false; // loss
                  })
                  .map((each) => each.count().toString())
                  .join("/")}
              </TableCell>
            </TableRow>
          </TableFooter>
        </Table>
      </div>
    </div>
  );
}

export default TeamPage;
