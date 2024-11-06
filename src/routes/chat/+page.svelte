<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Card from "$lib/components/ui/card";
  import { SendHorizontal } from "lucide-svelte";

  let messages: any[] = [];
  let inputMessage = "";
  let chatContainer: any;

  // Auto scroll to bottom when new messages arrive
  $: if (chatContainer && messages) {
    setTimeout(() => {
      chatContainer.scrollTop = chatContainer.scrollHeight;
    }, 0);
  }

  async function handleSubmit() {
    if (!inputMessage.trim()) return;

    // Add user message
    messages = [...messages, { role: "user", content: inputMessage }];

    // Clear input
    const messageToSend = inputMessage;
    inputMessage = "";

    try {
      // Add loading message
      messages = [
        ...messages,
        { role: "assistant", content: "...", loading: true },
      ];

      // Send message to API
      const response = await fetch("/api/chat", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ message: messageToSend }),
      });

      const data = await response.json();

      // Replace loading message with actual response
      messages = messages.slice(0, -1);
      messages = [...messages, { role: "assistant", content: data.message }];
    } catch (error) {
      console.error("Error:", error);
      messages = messages.slice(0, -1);
      messages = [
        ...messages,
        { role: "assistant", content: "Sorry, something went wrong." },
      ];
    }
  }
</script>

<div class="container mx-auto max-w-4xl p-4 space-y-4">
  <Card.Root class="h-[80vh]">
    <Card.Header>
      <Card.Title>Chat</Card.Title>
      <Card.Description>Chat with the AI assistant</Card.Description>
    </Card.Header>
    <Card.Content class="h-full pb-24 relative">
      <div
        class="messages h-full overflow-y-auto pr-4"
        bind:this={chatContainer}
      >
        {#each messages as message}
          <div class="message {message.role} mb-4">
            <div
              class="rounded-lg p-4 {message.role === 'user'
                ? 'bg-primary text-primary-foreground ml-auto'
                : 'bg-muted'} max-w-[80%] w-fit"
            >
              {#if message.loading}
                <div class="loading-dots">
                  <span>.</span><span>.</span><span>.</span>
                </div>
              {:else}
                {message.content}
              {/if}
            </div>
          </div>
        {/each}
      </div>

      <div class="absolute bottom-0 left-0 right-0 p-4 bg-card">
        <form on:submit|preventDefault={handleSubmit} class="flex gap-2">
          <Input
            type="text"
            bind:value={inputMessage}
            placeholder="Type your message..."
            class="flex-1"
          />
          <Button type="submit">
            <SendHorizontal class="h-4 w-4 mr-2" />
            Pattern
          </Button>
          <Button type="submit">
            <SendHorizontal class="h-4 w-4 mr-2" />
            Send
          </Button>
        </form>
      </div>
    </Card.Content>
  </Card.Root>
</div>

<style>
  .message.user {
    display: flex;
    justify-content: flex-end;
  }

  .loading-dots {
    display: flex;
    gap: 0.2rem;
  }

  .loading-dots span {
    animation: bounce 1s infinite;
  }

  .loading-dots span:nth-child(2) {
    animation-delay: 0.2s;
  }

  .loading-dots span:nth-child(3) {
    animation-delay: 0.4s;
  }

  @keyframes bounce {
    0%,
    100% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-5px);
    }
  }
</style>
