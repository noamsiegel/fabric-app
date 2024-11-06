<script lang="ts">
  // shadcn components
  import * as Card from "$lib/components/ui/card";
  import * as Drawer from "$lib/components/ui/drawer";
  import * as ContextMenu from "$lib/components/ui/context-menu/index.js";
  import * as Select from "$lib/components/ui/select";
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { buttonVariants } from "$lib/components/ui/button";
  import {
    Settings,
    Text,
    Link,
    Youtube,
    MessageCircleQuestion,
  } from "lucide-svelte/icons";
  // lib components
  import ModelParameters from "$lib/components/cards/ModelParameters.svelte";
  // tauri
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { create, BaseDirectory } from "@tauri-apps/plugin-fs";

  // Define the interface
  interface InputType {
    value: string;
    label: string;
    icon: any; // You could make this more specific based on your icon types
    flag: string;
  }

  // Input type options
  const inputTypes: InputType[] = [
    { value: "text", label: "Text", icon: Text, flag: "paste" },
    { value: "url", label: "URL", icon: Link, flag: "-u" },
    { value: "youtube", label: "YouTube", icon: Youtube, flag: "-y" },
    {
      value: "question",
      label: "Question",
      icon: MessageCircleQuestion,
      flag: "-q",
    },
  ];

  let messageType = $state(inputTypes[0]);

  // Define message type
  interface Message {
    role: "user" | "assistant";
    content: string;
    flag?: string;
    type?: string; // Optional since assistant messages won't have a type
  }

  // State management
  let messages = $state<Message[]>([
    { role: "assistant", content: "Hello! How can I help you today?" },
  ]);
  let inputMessage = $state("");
  let showSettings = $state(false);

  async function handleCopy(content: string) {
    try {
      console.log("Copying to clipboard:", content);
      await writeText(content);
    } catch (error) {
      console.error("Failed to copy to clipboard:", error);
    }
  }

  async function handleDownload(content: string) {
    try {
      // Generate timestamp for unique filename
      const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
      const filename = `fabric-chat-${timestamp}.txt`;

      // Create and write to file in Downloads directory
      const file = await create(filename, { baseDir: BaseDirectory.Download });
      await file.write(new TextEncoder().encode(content));
      await file.close();
    } catch (error) {
      console.error("Failed to download message:", error);
    }
  }

  // Update message display to show input type
  function getMessageClass(message: any) {
    const baseClass =
      message.role === "user"
        ? "ml-auto bg-primary/10 max-w-[80%]"
        : "mr-auto bg-muted max-w-[80%]";
    return baseClass;
  }

  // Update handleSend to properly manage message state
  function handleSend() {
    console.log("handleSend called");
    console.log("Current input:", inputMessage);
    if (inputMessage.trim()) {
      // Add user message
      messages = [
        ...messages,
        {
          role: "user",
          content: inputMessage,
          type: messageType.value,
          flag: messageType.flag,
        },
      ];

      // Add mock assistant response
      messages = [
        ...messages,
        {
          role: "assistant",
          content: "This is a mock response. Replace with actual AI response.",
        },
      ];

      // Clear input after sending
      inputMessage = "";
    }
  }
</script>

<div class="container mx-auto max-w-4xl p-4 space-y-4">
  <!-- Message Log -->
  <div class="space-y-4 h-[60vh] overflow-y-auto p-4 rounded-lg border">
    {#each messages as message}
      <ContextMenu.Root>
        <ContextMenu.Trigger>
          <Card.Root class={getMessageClass(message)}>
            <Card.Content class="p-3">
              {#if message.role === "user" && message.type}
                <div
                  class="flex items-center gap-2 mb-1 text-xs text-muted-foreground"
                >
                  {#if inputTypes.find((t) => t.value === message.type)?.icon}
                    {@const Icon = inputTypes.find(
                      (t) => t.value === message.type,
                    )?.icon}
                    <Icon class="size-3" />
                  {/if}
                  <span
                    >{inputTypes.find((t) => t.value === message.type)
                      ?.label}</span
                  >
                </div>
              {/if}
              <p class="text-sm">{message.content}</p>
            </Card.Content>
          </Card.Root>
        </ContextMenu.Trigger>
        <ContextMenu.Content>
          <ContextMenu.Item on:click={() => handleCopy(message.content)}>
            Copy
          </ContextMenu.Item>
          <ContextMenu.Item on:click={() => handleDownload(message.content)}>
            Download
          </ContextMenu.Item>
        </ContextMenu.Content>
      </ContextMenu.Root>
    {/each}
  </div>

  <!-- Input Area -->
  <div class="flex gap-2">
    <Select.Root bind:selected={messageType}>
      <Select.Trigger class="w-[140px]">
        <div class="flex items-center gap-2">
          {#if messageType.icon}
            {@const Icon = messageType.icon}
            <Icon class="size-4" />
          {/if}
          <Select.Value>{messageType.label}</Select.Value>
        </div>
      </Select.Trigger>
      <Select.Content>
        {#each inputTypes as type}
          <Select.Item value={type}>
            <div class="flex items-center gap-2">
              {#if type.icon}
                {@const Icon = type.icon}
                <Icon class="size-4" />
              {/if}
              {type.label}
            </div>
          </Select.Item>
        {/each}
      </Select.Content>
    </Select.Root>

    <!-- input -->
    <Input
      class="flex-1"
      placeholder="Type your message..."
      bind:value={inputMessage}
    />

    <!-- send button -->
    <Button on:click={handleSend}>Send</Button>

    <!-- settings drawer  -->
    <Drawer.Root bind:open={showSettings}>
      <Drawer.Trigger
        class={buttonVariants({ variant: "outline", size: "icon" })}
      >
        <Settings class="size-4" />
        <span class="sr-only">Model settings</span>
      </Drawer.Trigger>
      <Drawer.Content>
        <div class="mx-auto w-full max-w-sm">
          <Drawer.Header>
            <Drawer.Title>Model Parameters</Drawer.Title>
            <Drawer.Description>
              Adjust the AI model's behavior parameters.
            </Drawer.Description>
          </Drawer.Header>
          <ModelParameters />
          <Drawer.Footer>
            <Drawer.Close class={buttonVariants({ variant: "outline" })}>
              Close
            </Drawer.Close>
          </Drawer.Footer>
        </div>
      </Drawer.Content>
    </Drawer.Root>
  </div>
</div>
