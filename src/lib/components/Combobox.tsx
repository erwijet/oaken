import { useState } from "react";
import { Popover, PopoverTrigger, PopoverContent } from "@/lib/ui/popover";
import { Button } from "@/lib/ui/button";
import { Command, CommandInput, CommandEmpty, CommandGroup, CommandItem } from "@/lib/ui/command";
import { ChevronsUpDown, Check } from "lucide-react";
import { maybe } from "@tsly/maybe";

import { cn } from "@/lib/utils";

export interface ComboboxProps<T> {
  value: T | undefined;
  items: T[];
  placeholder?: string;
  disabled?: boolean;
  renderItemFn?: (item: T) => React.ReactNode;
  equalityFn?: (a: T, b: T) => boolean;
  onChange?: (selected: T | undefined) => unknown;
  intoQueryable: (item: T) => string;
}

export function Combobox<T>(props: ComboboxProps<T>) {
  const [open, setOpen] = useState(false);

  const isEq = props.equalityFn ?? ((a: T, b: T) => a?.toString() ?? a == b?.toString() ?? b);
  const render = props.renderItemFn ?? ((item: T) => item?.toString() ?? JSON.stringify({ item }));

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild disabled={props.disabled}>
        <Button variant="outline" role="combobox" aria-expanded={open} className="w-[200px] justify-between">
          {maybe(props.value)
            ?.let((it) => props.items.find((item) => isEq(item, it)))
            ?.take(render) ??
            props.placeholder ??
            "Select..."}
          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[200px] p-0">
        <Command filter={(idxStr, query) => props.intoQueryable(props.items.at(parseInt(idxStr))!).includes(query) ? 1 : 0}>
          <CommandInput placeholder="Search..." />
          <CommandEmpty>Nothing found.</CommandEmpty>
          <CommandGroup>
            {props.items.map((item, i) => (
              <CommandItem
                key={i}
                value={i.toString()}
                onSelect={(cur) => {
                  props.onChange?.(!!cur ? item : undefined);
                  setOpen(false);
                }}
              >
                <Check className={cn("mr-2 h-4 w-4", maybe(props.value)?.take((it) => isEq(it, item)) ? "opacity-100" : "opacity-0")} />
                {render(item)}
              </CommandItem>
            ))}
          </CommandGroup>
        </Command>
      </PopoverContent>
    </Popover>
  );
}
