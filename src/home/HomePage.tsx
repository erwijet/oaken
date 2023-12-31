import { useQuery, useQueryClient } from "@tanstack/react-query";
import { api } from "@/lib/rpc";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/lib/ui/table";
import { Badge } from "../lib/ui/badge";
import { TeamSelect } from "../lib/prefab/TeamSelect";
import { Button } from "../lib/ui/button";
import { AtSign, Swords } from "lucide-react";
import { maybe } from "@tsly/maybe";
import { useNavigate } from "react-router";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "../lib/ui/tabs";
import { SnapshotView } from "./SnapshotView";
import { ScheduleView } from "./ScheduleView";
import { StandingsView } from "./StandingsView";

export function TabControl() {
  return (
    <TabsList className="mr-4">
      <TabsTrigger value="snapshot">Snapshot</TabsTrigger>
      <TabsTrigger value="schedule">Schedule</TabsTrigger>
      <TabsTrigger value="standings">Standings</TabsTrigger>
    </TabsList>
  );
}

function HomePage() {
  const { data: teams } = useQuery({
    queryKey: ["getTeams"],
    queryFn: () => api.query(["getTeams"]),
  });

  const { data: game } = useQuery({
    queryKey: ["getGameState"],
    queryFn: () => api.query(["getGameState"]),
  });

  const client = useQueryClient();
  const nav = useNavigate();

  return (
    <div className="p-4 w-full h-full flex flex-col gap-4">
      <div className="flex justify-between w-full">
        <div className="flex gap-2 items-center">
          <h1 className="text-4xl font-extrabold tracking-tight self-start">Gameday</h1>
          <span>
            <Badge>Season: {game?.year}</Badge>
          </span>
          <span>
            <Badge>Week: {game?.wk_no}</Badge>
          </span>
        </div>
        <div className="flex gap-2">
          <TeamSelect />
          <Button onClick={() => api.mutation(["advanceWeek"]).then(() => client.invalidateQueries())}>
            Next Week
            <Swords className="w-4 h-4 ml-2" />
          </Button>
        </div>
      </div>

      <hr />

      <Tabs defaultValue="snapshot" className="w-full">
        <TabsContent value="snapshot">
          <SnapshotView />
        </TabsContent>

        <TabsContent value="schedule">
          <ScheduleView />
        </TabsContent>

        <TabsContent value="standings">
          <StandingsView />
        </TabsContent>
      </Tabs>
    </div>
  );
}

export default HomePage;
