<script lang="ts">
    import * as Table from "$lib/components/ui/table";
    import type { Employee } from "$lib/types/employee";
    import { calculateAge, getInitials } from "$lib/utils";
    import type { Snippet } from "svelte";
    import StatusBadge from "../StatusBadge.svelte";
    import AvatarFallback from "../ui/avatar/avatar-fallback.svelte";
    import AvatarImage from "../ui/avatar/avatar-image.svelte";
    import Avatar from "../ui/avatar/avatar.svelte";
    import { Phone } from "lucide-svelte";

    interface Props {
        employees: Employee[];
        actions: Snippet<[number]>;
        pagination: Snippet;
    }

    const { employees, actions, pagination }: Props = $props();

    function parsePhoneNumbers(phoneJson: string | undefined): string[] {
        if (!phoneJson) return [];
        try {
            return JSON.parse(phoneJson);
        } catch {
            return [];
        }
    }
</script>

<div class=" border border-gray-300 rounded-md overflow-hidden">
    <Table.Root>
        <Table.Header class="bg-gray-200">
            <Table.Row>
                <Table.Head>Employee</Table.Head>
                <Table.Head>Status</Table.Head>
                <Table.Head>Job Post</Table.Head>
                <Table.Head>Current Address</Table.Head>
                <Table.Head>Post</Table.Head>
                <Table.Head>Phone</Table.Head>
                <Table.Head>Age</Table.Head>
                <Table.Head>Joining Date</Table.Head>
                <Table.Head>Actions</Table.Head>
            </Table.Row>
        </Table.Header>
        <Table.Body>
            {#each employees as employee (employee.id)}
                <Table.Row>
                    <Table.Cell class="border border-gray-300">
                        <div class="flex items-center gap-3">
                            <Avatar>
                                {#if employee.photoPath}
                                    <AvatarImage
                                        src={employee.photoPath}
                                        alt={employee.name}
                                    />
                                {/if}
                                <AvatarFallback
                                    >{getInitials(
                                        employee.name,
                                    )}</AvatarFallback
                                >
                            </Avatar>
                            <div>
                                <div class="font-semibold">{employee.name}</div>
                                <div class="text-xs text-gray-500">
                                    {employee.essid || "No ESSID"}
                                </div>
                            </div>
                        </div>
                    </Table.Cell>
                    <Table.Cell class="border border-gray-300"
                        ><StatusBadge
                            status={employee.employmentStatus}
                        /></Table.Cell
                    >
                    <Table.Cell class="border border-gray-300"
                        >{employee.jobPost}</Table.Cell
                    >
                    <Table.Cell class="border border-gray-300"
                        >{employee.currentAddress}</Table.Cell
                    >
                    <Table.Cell class="border border-gray-300"
                        >{employee.permanentPost}</Table.Cell
                    >
                    <Table.Cell class="border border-gray-300">
                        <div class="text-sm space-y-1">
                            {#each parsePhoneNumbers(employee.phoneNumbers).slice(0, 2) as phone}
                                <div class="flex items-center gap-1">
                                    <Phone class="h-3 w-3 text-gray-400" />
                                    {phone}
                                </div>
                            {/each}
                            {#if parsePhoneNumbers(employee.phoneNumbers).slice(0, 2).length > 2}
                                <div class="text-xs text-gray-500">
                                    +{parsePhoneNumbers(
                                        employee.phoneNumbers,
                                    ).slice(0, 2).length - 2} more
                                </div>
                            {/if}
                        </div></Table.Cell
                    >
                    <Table.Cell class="border border-gray-300"
                        >{employee.dateOfBirth &&
                            calculateAge(employee.dateOfBirth)}</Table.Cell
                    >
                    <Table.Cell class="border border-gray-300"
                        >{employee.joiningDate}</Table.Cell
                    >
                    <Table.Cell class="border border-gray-300">
                        {@render actions(employee.id)}
                    </Table.Cell>
                </Table.Row>
            {/each}
        </Table.Body>
    </Table.Root>
    {@render pagination()}
</div>
