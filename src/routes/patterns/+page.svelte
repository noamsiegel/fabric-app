<script lang="ts">
  import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";
  import { Button } from "$lib/components/ui/button";

  let patterns = [
    { id: 1, name: "Small Talk", description: "General conversation starters" },
    {
      id: 2,
      name: "Technical Support",
      description: "Troubleshooting common issues",
    },
    // Add more patterns as needed
  ];
</script>

<div class="p-4">
  <h1 class="text-2xl font-bold mb-4">Pattern Browser</h1>

  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
    {#each patterns as pattern}
      <Card>
        <CardHeader>
          <CardTitle>{pattern.name}</CardTitle>
          <CardDescription>{pattern.description}</CardDescription>
        </CardHeader>
        <CardFooter>
          <Button variant="outline">Select</Button>
        </CardFooter>
      </Card>
    {/each}
  </div>
</div>
