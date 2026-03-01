<script lang="ts">
    import { Input } from "$lib/components/ui/input";
    import { Button } from "$lib/components/ui/button";
    import * as Select from "$lib/components/ui/select";
    import * as Card from "$lib/components/ui/card";
    import { Search, X, Calendar, MapPinHouseIcon } from "lucide-svelte";
    import { EmployeeStatus, type FilterOptions } from "$lib/types/employee";

    interface Props {
        filters: FilterOptions;
    }

    let { filters = $bindable() }: Props = $props();

    function handleReset() {
        filters = {};
    }

    const employmentStatusOptions = [
        { value: "", label: "Status: All" },
        { value: EmployeeStatus.APPLIED, label: "Applied" },
        { value: EmployeeStatus.CURRENT, label: "Current" },
        { value: EmployeeStatus.PAST, label: "Past" },
    ];
</script>

<Card.Root class="shadow-sm border-muted">
    <Card.Content class="p-2">
        <div class="flex flex-wrap gap-2 items-center">
            <div class="relative flex-[1_1_200px] min-w-50">
                <Search
                    class="absolute left-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-muted-foreground"
                />
                <Input
                    type="text"
                    bind:value={filters.query}
                    placeholder="Search name or ESSID..."
                    class="h-8 pl-8 text-sm bg-background"
                />
            </div>

            <div class="w-32.2 shrink-0">
                <Select.Root
                    type="single"
                    name="employmentStatus"
                    bind:value={filters.employmentStatus}
                >
                    <Select.Trigger class="h-8 w-full text-sm">
                        <span class="truncate">
                            {employmentStatusOptions.find(
                                (o) => o.value === filters.employmentStatus,
                            )?.label || "Status"}
                        </span>
                    </Select.Trigger>
                    <Select.Content>
                        {#each employmentStatusOptions as option}
                            <Select.Item value={option.value} class="text-sm">
                                {option.label}
                            </Select.Item>
                        {/each}
                    </Select.Content>
                </Select.Root>
            </div>

            <div class="relative w-32.2 shrink-0">
                <MapPinHouseIcon
                    class="absolute left-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-muted-foreground"
                />
                <Input
                    type="text"
                    bind:value={filters.post}
                    placeholder="Location"
                    class="h-8 pl-8 text-sm"
                />
            </div>

            <div
                class="flex items-center h-8 flex-[1_1_250px] min-w-62.5 rounded-md border border-input bg-background px-2.5 shadow-sm focus-within:ring-1 focus-within:ring-ring overflow-hidden"
            >
                <Calendar
                    class="mr-1.5 h-3.5 w-3.5 text-muted-foreground shrink-0"
                />
                <span
                    class="text-[10px] uppercase font-semibold text-muted-foreground tracking-wider mr-2 shrink-0"
                    >Joined</span
                >
                <input
                    type="text"
                    onfocus={(e) => (e.currentTarget.type = "date")}
                    onblur={(e) => (e.currentTarget.type = "text")}
                    bind:value={filters.joiningDateFrom}
                    placeholder="From"
                    class="w-full min-w-15 bg-transparent text-sm outline-none placeholder:text-muted-foreground/60 border border-gray-300 rounded-sm"
                />
                <span class="text-muted-foreground/40 mx-1">-</span>
                <input
                    type="text"
                    onfocus={(e) => (e.currentTarget.type = "date")}
                    onblur={(e) => (e.currentTarget.type = "text")}
                    bind:value={filters.joiningDateTo}
                    placeholder="To"
                    class="w-full min-w-15 bg-transparent text-sm outline-none placeholder:text-muted-foreground/60 border border-gray-300 rounded-sm"
                />
            </div>

            <div
                class="flex items-center h-8 flex-[1_1_250px] min-w-62.5 rounded-md border border-input bg-background px-2.5 shadow-sm focus-within:ring-1 focus-within:ring-ring overflow-hidden"
            >
                <Calendar
                    class="mr-1.5 h-3.5 w-3.5 text-muted-foreground shrink-0"
                />
                <span
                    class="text-[10px] uppercase font-semibold text-muted-foreground tracking-wider mr-2 shrink-0"
                    >Exited</span
                >
                <input
                    type="text"
                    onfocus={(e) => (e.currentTarget.type = "date")}
                    onblur={(e) => (e.currentTarget.type = "text")}
                    bind:value={filters.exitDateFrom}
                    placeholder="From"
                    class="w-full min-w-15 bg-transparent text-sm outline-none placeholder:text-muted-foreground/60 border border-gray-300 rounded-sm"
                />
                <span class="text-muted-foreground/40 mx-1">-</span>
                <input
                    type="text"
                    onfocus={(e) => (e.currentTarget.type = "date")}
                    onblur={(e) => (e.currentTarget.type = "text")}
                    bind:value={filters.exitDateTo}
                    placeholder="To"
                    class="w-full min-w-15 bg-transparent text-sm outline-none placeholder:text-muted-foreground/60 border border-gray-300 rounded-sm"
                />
            </div>

            <Button
                variant="ghost"
                size="icon"
                onclick={handleReset}
                class="h-8 w-8 ml-auto text-muted-foreground hover:bg-destructive/10 hover:text-destructive shrink-0"
                title="Clear Filters"
            >
                <X class="h-4 w-4" />
            </Button>
        </div>
    </Card.Content>
</Card.Root>
