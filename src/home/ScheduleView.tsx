import { api } from "@/lib/rpc";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/lib/ui/table";
import { useQuery } from "@tanstack/react-query";
import { maybe } from "@tsly/maybe";
import { AtSign } from "lucide-react";
import { Button } from "../lib/ui/button";
import { TabControl } from "./HomePage";
import { useNavigate } from "react-router";

export function ScheduleView() {
  const { data: game } = useQuery({
    queryKey: ["getGameState"],
    queryFn: () => api.query(["getGameState"]),
  });

  const { data: matchups } = useQuery({
    queryKey: ["getSchedules", game?.year ?? 0],
    enabled: !!game,
    queryFn: () =>
      api
        .query(["getSchedules"])
        .then((schedules) => {
          console.log(schedules);
          return schedules;
        })
        .then((schedules) => schedules.find((schedule) => schedule.year == game!.year)?.matchups),
  });

  const { data: teams } = useQuery({
    queryKey: ["getTeams"],
    queryFn: () => api.query(["getTeams"]),
  });

  const nav = useNavigate();

  return (
    <>
      <div className="w-full flex justify-between items-center mb-4">
        <h3 className="text-2xl text-gray-700 font-bold ml-4">Season Schedule</h3>
        <TabControl />
      </div>
      <div className="w-[calc(100%-2rem)] mx-4 border-px rounded-md shadow-lg">
        <Table>
          <TableHeader>
            <TableHead>Week</TableHead>
            <TableHead>Away Team</TableHead>
            <TableHead>Home Team</TableHead>
            <TableHead>Results</TableHead>
          </TableHeader>
          <TableBody>
            {matchups?.map((matchup) => (
              <TableRow>
                <TableCell>{matchup.wkNo}</TableCell>
                <TableCell>
                  {maybe(teams?.find((team) => team.id == matchup.awayTeamId))?.take((team) => (
                    <Button variant={"link"} onClick={() => nav("/team/" + team.id)}>
                      {team.name}
                    </Button>
                  ))}
                </TableCell>
                <TableCell>
                  {maybe(teams?.find((team) => team.id == matchup.homeTeamId))?.take((team) => (
                    <Button variant={"link"} onClick={() => nav("/team/" + team.id)}>
                      <AtSign className="h-4 w-4 mr-2" />
                      {team.name}
                    </Button>
                  ))}
                </TableCell>
                <TableCell>
                  {maybe(matchup.awayTeamScore)?.take(
                    (awayTeamScore) => maybe(matchup.homeTeamScore)?.take((homeTeamScore) => `${awayTeamScore} - ${homeTeamScore}`),
                  ) ?? "-"}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>
    </>
  );
}
