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

	interface Pattern {
		value: string;
		label: string;
	}

	let patterns = $state<Pattern[]>([]);
	let open = $state(false);
	let value = $state("");
	let search = $state("");
	let triggerRef = $state<HTMLButtonElement>(null!);

	const selectedValue = $derived(
		patterns.find((p) => p.value === value)?.label ?? "Select a pattern...",
	);

	async function getPatterns() {
		const result = await invoke("get_patterns");
		patterns = (result as string[]).map((pattern) => ({
			value: pattern,
			label: pattern,
		}));
	}

	function closeAndFocusTrigger() {
		open = false;
		tick().then(() => {
			triggerRef?.focus();
		});
	}

	onMount(() => {
		getPatterns();
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
					placeholder="Search pattern..."
					bind:value={search}
				/>
				{#key search}
					<Command.List>
						<Command.Empty>No pattern found.</Command.Empty>
						<Command.Group>
							{#each patterns.filter((p) => p.label
									.toLowerCase()
									.includes(search.toLowerCase())) as pattern}
								<Command.Item
									value={pattern.value}
									onSelect={() => {
										value = pattern.value;
										closeAndFocusTrigger();
									}}
								>
									<Check
										class={cn(
											"mr-2 h-4 w-4",
											value !== pattern.value &&
												"text-transparent",
										)}
									/>
									{pattern.label}
								</Command.Item>
							{/each}
						</Command.Group>
					</Command.List>
				{/key}
			</Command.Root>
		</Popover.Content>
	</Popover.Root>
</div>
