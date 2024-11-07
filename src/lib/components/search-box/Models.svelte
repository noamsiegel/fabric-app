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

	let { onModelSelected = (model: string) => {} } = $props<{
		onModelSelected?: (model: string) => void;
	}>();

	interface Model {
		value: string;
		label: string;
		provider: string;
	}

	let models = $state<Model[]>([]);
	let open = $state(false);
	let value = $state("");
	let search = $state("");
	let triggerRef = $state<HTMLButtonElement>(null!);

	const selectedValue = $derived(
		models.find((m) => m.value === value)?.label ?? "Select a model...",
	);

	async function getModels() {
		try {
			const result = await invoke("get_models");
			models = (result as any[]).map((model) => ({
				value: model.name,
				label: model.name,
				provider: model.provider,
			}));
		} catch (error) {
			console.error("Failed to list models:", error);
		}
	}

	function closeAndFocusTrigger() {
		open = false;
		tick().then(() => {
			triggerRef?.focus();
		});
	}

	onMount(() => {
		getModels();
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
				{selectedValue}
				<ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
			</Button>
		</Popover.Trigger>
		<Popover.Content class="w-[300px] p-0">
			<Command.Root>
				<Command.Input
					placeholder="Search model..."
					bind:value={search}
				/>
				{#key search}
					<Command.List>
						<Command.Empty>No model found.</Command.Empty>
						<Command.Group>
							{#each models.filter((m) => m.label
										.toLowerCase()
										.includes(search.toLowerCase()) || m.provider
										.toLowerCase()
										.includes(search.toLowerCase())) as model}
								<Command.Item
									value={model.value}
									onSelect={() => {
										value = model.value;
										onModelSelected(model.value);
										closeAndFocusTrigger();
									}}
								>
									<Check
										class={cn(
											"mr-2 h-4 w-4",
											value !== model.value &&
												"text-transparent",
										)}
									/>
									{model.label}
									<span
										class="ml-2 text-sm text-muted-foreground"
									>
										({model.provider})
									</span>
								</Command.Item>
							{/each}
						</Command.Group>
					</Command.List>
				{/key}
			</Command.Root>
		</Popover.Content>
	</Popover.Root>
</div>
