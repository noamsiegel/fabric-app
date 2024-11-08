<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import Check from "lucide-svelte/icons/check";
	import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
	import * as Command from "$lib/components/ui/command";
	import * as Popover from "$lib/components/ui/popover";
	import { Button } from "$lib/components/ui/button";
	import { cn } from "$lib/utils";
	import { tick } from "svelte";
	import { onMount } from "svelte";
	import { defaultVendorStore } from "$lib/stores/models";
	import { ModelSettingsManager } from "$lib/managers/ModelManager";

	let { onVendorSelected = () => {} } = $props<{
		onVendorSelected?: (vendor: string) => void;
	}>();

	const manager = new ModelSettingsManager();

	interface Vendor {
		value: string;
		label: string;
	}

	let vendors = $state<Vendor[]>([]);
	let open = $state(false);
	let value = $state("");
	let search = $state("");
	let triggerRef = $state<HTMLButtonElement>(null!);

	const selectedVendor = $derived(
		vendors.find((v) => v.value === value)?.label ??
			$defaultVendorStore ??
			"Select a vendor...",
	);

	async function getVendors() {
		try {
			const result = await invoke("get_vendors");
			vendors = (result as string[]).map((vendor) => ({
				value: vendor,
				label: vendor,
			}));
		} catch (error) {
			console.error("Failed to get vendors:", error);
		}
	}

	async function loadDefaultVendor() {
		await manager.loadDefaultVendor();
		if (manager.currentVendor) {
			value = manager.currentVendor;
			onVendorSelected(manager.currentVendor);
		}
	}

	function closeAndFocusTrigger() {
		open = false;
		tick().then(() => {
			triggerRef?.focus();
		});
	}

	onMount(async () => {
		await getVendors();
		await loadDefaultVendor();
	});
</script>

<div class="flex flex-col gap-4">
	<Popover.Root bind:open>
		<Popover.Trigger bind:ref={triggerRef}>
			<Button
				variant="outline"
				role="combobox"
				aria-expanded={open}
				class="w-[300px] justify-between"
			>
				{selectedVendor}
				<ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
			</Button>
		</Popover.Trigger>
		<Popover.Content class="w-[300px] p-0">
			<Command.Root>
				<Command.Input
					placeholder="Search vendor..."
					bind:value={search}
				/>
				{#key search}
					<Command.List>
						<Command.Empty>No vendor found.</Command.Empty>
						<Command.Group>
							{#each vendors.filter((v) => v.label
									.toLowerCase()
									.includes(search.toLowerCase())) as vendor}
								<Command.Item
									value={vendor.value}
									onSelect={async () => {
										value = vendor.value;
										await manager.saveDefaultVendor(
											vendor.value,
										); // Save as default
										onVendorSelected(vendor.value);
										closeAndFocusTrigger();
									}}
								>
									<Check
										class={cn(
											"mr-2 h-4 w-4",
											value !== vendor.value &&
												"text-transparent",
										)}
									/>
									{vendor.label}
								</Command.Item>
							{/each}
						</Command.Group>
					</Command.List>
				{/key}
			</Command.Root>
		</Popover.Content>
	</Popover.Root>
</div>
