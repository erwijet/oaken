import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/rpc";
import { Badge } from "../lib/ui/badge";
import { TeamSelect } from "../lib/prefab/TeamSelect";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "../lib/ui/tabs";
import { SnapshotView } from "./SnapshotView";
import { ScheduleView } from "./ScheduleView";
import { StandingsView } from "./StandingsView";
import { Combobox } from "@/lib/components/Combobox";
import { maybe } from "@tsly/maybe";
import { LeagueInfo, Tier } from "@/bindings";
import { useGlobalState } from "@/lib/utils";
import { NextWeekButton } from "@/lib/prefab/NextWeekButton";
import { useEventHandler } from "@/lib/events";

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
  const [tierId, setTierId] = useGlobalState<number>("tierId");
  const [leagueId, setLeagueId] = useGlobalState<number>("leagueId");

  useEventHandler("game_did_restart", () => {
    // tierIds and leagueIds are regenerated when the game restarts,
    // so we clear them here in case they were set from the previous game
    setTierId(undefined);
    setLeagueId(undefined);
  });

  const { data: game } = useQuery({
    queryKey: ["getGameState"],
    queryFn: () => api.query(["getGameState"]),
  });

  const { data: divisions } = useQuery({
    queryKey: ["getLeagueInfos"],
    queryFn: () => api.query(["getLeagueInfos"]),
    select: (leagueInfos) => leagueInfos.flatMap((league) => league.tiers.map((tier) => ({ tier, league }))),
  });

  function resolveDivisionIds(ids: { tierId?: number; leagueId?: number }, by: { league: LeagueInfo; tier: Tier }[]) {
    return by.find((it) => it.league.id == ids.leagueId && it.tier.id == ids.tierId);
  }

  return (
    <div className="p-4 w-full h-full flex flex-col gap-4">
      <div className="flex justify-between w-full">
        <div className="flex gap-2 items-center flex-col">
          <h1 className="text-4xl font-extrabold tracking-tight self-start">Gameday</h1>

          <div className="flex gap-2 items-center">
            <span>
              <Badge>Season: {game?.year}</Badge>
            </span>
            <span>
              <Badge>Week: {game?.wk_no}</Badge>
            </span>
          </div>
        </div>
        <div className="flex gap-2">
          <TeamSelect />
          <NextWeekButton />
        </div>
      </div>

      <div className="flex justify-end items-center w-full">
        <Combobox
          items={divisions?.map((each) => ({ leagueId: each.league.id, tierId: each.tier.id })) ?? []}
          intoQueryable={(ids) => maybe(resolveDivisionIds(ids, divisions ?? []))?.take((it) => `${it.league.name} ${it.tier.name}`) ?? ""}
          value={{ leagueId, tierId }}
          equalityFn={(a, b) => a.leagueId == b.leagueId && a.tierId == b.tierId}
          onChange={(ids) => {
            setLeagueId(ids?.leagueId);
            setTierId(ids?.tierId);
          }}
          renderItemFn={(ids) =>
            maybe(resolveDivisionIds(ids, divisions ?? []))?.take((it) => (
              <span>
                {it.league.name}, {it.tier.name}
              </span>
            ))
          }
        />
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
