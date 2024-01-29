import { api } from "@/lib/rpc";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/lib/ui/table";
import { useQuery } from "@tanstack/react-query";
import { TabControl } from "./HomePage";
import { useGlobalState } from "@/lib/utils";
import { Button } from "@/lib/ui/button";
import { useNavigate } from "react-router";

export function StandingsView() {
  const [tierId] = useGlobalState("tierId");
  const [leagueId] = useGlobalState("leagueId");

  const { data: game } = useQuery({
    queryKey: ["getGameState"],
    queryFn: () => api.query(["getGameState"]),
  });

  const { data: standings } = useQuery({
    enabled: !!game,
    queryKey: ["getStandings", game?.year ?? 0],
    queryFn: () => api.query(["getStandings", game!.year]),
    select: (standings) => standings.filter((it) => it.leagueId == leagueId && it.tierId == tierId),
  });

  const nav = useNavigate();

  return (
    <>
      <div className="w-full flex justify-between items-center mb-4">
        <h3 className="text-2xl text-gray-700 font-bold ml-4">{game?.year} Standings</h3>
        <TabControl />
      </div>
      <div className="w-[calc(100%-2rem)] mx-4 border-px rounded-md shadow-lg">
        <Table>
          <TableHeader>
            <TableHead>Pos.</TableHead>
            <TableHead className="w-[99%]">Team</TableHead>
            <TableHead>
              <abbr title="Wins">W</abbr>
            </TableHead>
            <TableHead>
              <abbr title="Losses">L</abbr>
            </TableHead>
            <TableHead>
              <abbr title="Draws">T</abbr>
            </TableHead>
            <TableHead>
              <abbr title="Win Percentage">%</abbr>
            </TableHead>
            <TableHead>
              <abbr title="Points For">PF</abbr>
            </TableHead>
            <TableHead>
              <abbr title="Points Against">PA</abbr>
            </TableHead>
            <TableHead>
              <abbr title="Streak">Strk</abbr>
            </TableHead>
          </TableHeader>
          <TableBody>
            {standings?.map((each, i) => (
              <TableRow className="hover:bg-inherit">
                <TableCell>{i + 1}.</TableCell>
                <TableCell className="p-0">
                  <Button variant={"link"} onClick={() => nav(`/team/${each.teamId}`)}>
                    {each.teamName}
                  </Button>
                </TableCell>
                <TableCell>{each.wins}</TableCell>
                <TableCell>{each.losses}</TableCell>
                <TableCell>{each.draws}</TableCell>
                <TableCell>{each.winPercent?.toFixed(3) ?? "-"}</TableCell>
                <TableCell>{each.pointsFor}</TableCell>
                <TableCell>{each.pointsAgainst}</TableCell>
                <TableCell>{each.streak}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>
    </>
  );
}
