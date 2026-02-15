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
        { value: "", label: "All Statuses" },
        { value: EmployeeStatus.APPLIED, label: "Applied" },
        { value: EmployeeStatus.CURRENT, label: "Current" },
        { value: EmployeeStatus.PAST, label: "Past" },
    ];
</script>

<Card.Root class="shadow-sm border-muted">
    <Card.Content class="p-3">
        <div class="flex flex-col xl:flex-row gap-3">
            <div class="relative flex-1 min-w-50">
                <Search
                    class="absolute left-2.5 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground"
                />
                <Input
                    type="text"
                    bind:value={filters.query}
                    placeholder="Search by name or ESSID"
                    class="pl-9 h-9 bg-background"
                />
            </div>

            <div
                class="grid grid-cols-2 md:grid-cols-4 xl:flex gap-2 flex-wrap items-center"
            >
                <div class="w-full md:w-40">
                    <Select.Root
                        type="single"
                        name="employmentStatus"
                        bind:value={filters.employmentStatus}
                    >
                        <Select.Trigger class="h-9 w-full">
                            <span class="truncate">
                                {employmentStatusOptions.find(
                                    (o) => o.value === filters.employmentStatus,
                                )?.label || "Status"}
                            </span>
                        </Select.Trigger>
                        <Select.Content>
                            {#each employmentStatusOptions as option}
                                <Select.Item value={option.value}
                                    >{option.label}</Select.Item
                                >
                            {/each}
                        </Select.Content>
                    </Select.Root>
                </div>

                <!-- Job Position -->
                <!-- <div class="relative w-full md:w-[160px]">
                    <BriefcaseBusinessIcon
                        class="absolute left-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-muted-foreground"
                    />
                    <Input
                        type="text"
                        bind:value={filters.jobPost}
                        placeholder="Position"
                        class="pl-8 h-9"
                    />
                </div> -->

                <div class="relative w-full md:w-35">
                    <MapPinHouseIcon
                        class="absolute left-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-muted-foreground"
                    />
                    <Input
                        type="text"
                        bind:value={filters.post}
                        placeholder="Place"
                        class="pl-8 h-9"
                    />
                </div>

                <div class="relative w-full md:w-35">
                    <Calendar
                        class="absolute left-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-muted-foreground pointer-events-none"
                    />
                    <Input
                        type="text"
                        onfocus={(e) => (e.currentTarget.type = "date")}
                        onblur={(e) => (e.currentTarget.type = "text")}
                        bind:value={filters.joiningDate}
                        placeholder="Joined"
                        class="pl-8 h-9 w-full"
                    />
                </div>

                <div class="relative w-full md:w-35">
                    <Calendar
                        class="absolute left-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-muted-foreground pointer-events-none"
                    />
                    <Input
                        type="text"
                        onfocus={(e) => (e.currentTarget.type = "date")}
                        onblur={(e) => (e.currentTarget.type = "text")}
                        bind:value={filters.exitDate}
                        placeholder="Exited"
                        class="pl-8 h-9 w-full"
                    />
                </div>
            </div>

            <Button
                variant="ghost"
                size="icon"
                onclick={handleReset}
                class="h-9 w-9 text-muted-foreground hover:text-destructive shrink-0 ml-auto xl:ml-0"
                title="Reset Filters"
            >
                <X class="h-4 w-4" />
            </Button>
        </div>
    </Card.Content>
</Card.Root>
