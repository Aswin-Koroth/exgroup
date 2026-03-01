<script lang="ts">
    import { toast } from "svelte-sonner";
    import type { Snippet } from "svelte";
    import { Copy, Phone } from "lucide-svelte";
    import Avatar from "../ui/avatar/avatar.svelte";
    import StatusBadge from "../StatusBadge.svelte";
    import * as Table from "$lib/components/ui/table";
    import type { Employee } from "$lib/types/employee";
    import { convertFileSrc } from "@tauri-apps/api/core";
    import AvatarImage from "../ui/avatar/avatar-image.svelte";
    import AvatarFallback from "../ui/avatar/avatar-fallback.svelte";
    import { calculateAge, copyToClipboard, getInitials } from "$lib/utils";

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

    function handlePhoneCopy(phone: string) {
        copyToClipboard(phone);
        toast.success("Phone number copied to clipboard");
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
                                <AvatarImage
                                    src={employee.photoPath
                                        ? convertFileSrc(employee.photoPath)
                                        : ""}
                                    alt={employee.name}
                                />
                                <AvatarFallback
                                    >{getInitials(
                                        employee.name,
                                    )}</AvatarFallback
                                >
                            </Avatar>
                            <div>
                                <div class="font-semibold">{employee.name}</div>
                                <div class="text-xs text-gray-500">
                                    {#if employee.essid}
                                        ESSID: <span class="font-semibold"
                                            >{employee.essid}</span
                                        >
                                    {:else}
                                        No ESSID
                                    {/if}
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
                                    <Copy
                                        class="h-3 w-3 text-gray-400 hover:text-gray-600"
                                        onclick={() => handlePhoneCopy(phone)}
                                    />
                                </div>
                            {/each}
                            {#if parsePhoneNumbers(employee.phoneNumbers).length > 2}
                                <div class="text-xs text-gray-500">
                                    +{parsePhoneNumbers(employee.phoneNumbers)
                                        .length - 2} more
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
