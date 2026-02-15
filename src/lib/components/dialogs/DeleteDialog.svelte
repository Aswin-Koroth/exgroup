<script lang="ts">
    import * as AlertDialog from "$lib/components/ui/alert-dialog";

    interface Props {
        deleteDialogOpen: boolean;
        onConfirmDelete: () => Promise<void>;
    }

    let { deleteDialogOpen = $bindable(), onConfirmDelete }: Props = $props();
</script>

<AlertDialog.Root bind:open={deleteDialogOpen}>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title>Are you absolutely sure?</AlertDialog.Title>
            <AlertDialog.Description>
                This action cannot be undone. This will permanently delete the
                employee record.
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
            <AlertDialog.Action
                class="bg-red-600 text-white hover:bg-red-700"
                onclick={onConfirmDelete}
            >
                Delete
            </AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>
