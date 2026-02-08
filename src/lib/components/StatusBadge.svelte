<script lang="ts">
    import { Badge } from "$lib/components/ui/badge";
    import { EmployeeStatus } from "$lib/types/employee";
    import { UserCheck, UserX, UserPlus } from "lucide-svelte";

    interface Props {
        status: EmployeeStatus;
    }

    let { status }: Props = $props();

    const statusConfig = {
        applied: {
            label: "Applied",
            variant: "secondary" as const,
            icon: UserPlus,
            class: "bg-blue-100 text-blue-800 hover:bg-blue-100",
        },
        current: {
            label: "Current",
            variant: "default" as const,
            icon: UserCheck,
            class: "bg-green-100 text-green-800 hover:bg-green-100",
        },
        past: {
            label: "Past",
            variant: "outline" as const,
            icon: UserX,
            class: "bg-gray-100 text-gray-800 hover:bg-gray-100",
        },
    };

    const config = $derived(statusConfig[status]);
</script>

<Badge variant={config.variant} class={config.class}>
    <config.icon class="h-3 w-3 mr-1" />
    {config.label}
</Badge>
