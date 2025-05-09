import { Command } from "@tauri-apps/plugin-shell";
import { invoke } from "@tauri-apps/api/core";

// base fabric settings
export async function runFabric(flag: string, message: string, model: string, pattern: string, context?: string) {
  try {
    if (pattern === "list-patterns") {
      // Args for the original fabric command (if needed for context, but not executed)
      const fabricArgsForPiping = [
        flag,
        message,
        "|",
        "fabric", 
        "--pattern",
        pattern,
        "--model",
        model,
      ];
      if (context && context.trim()) {
        // fabricArgsForPiping.push("--context", context); // Not needed for echo test
      }

      console.log("Attempting to run /bin/echo command...");
      const testEchoArgs = ["Hello from Tauri Shell"];
      const result = await Command.create("echo", testEchoArgs).execute();
      
      console.log("Echo command stdout:", result.stdout);
      console.log("Echo command stderr:", result.stderr);
      console.log("Echo command exit code:", result.code); // 'code' is often used for exit status
      // console.log("Full echo result object:", result); // For deeper inspection if needed

      // Return echo's stdout for this test
      return result.stdout;
    } else {
      // Construct arguments for the main fabric command
      const fabricRunArgs: string[] = [
        flag,
        message,
      ];

      // This is the tricky part. If your `fabric` command literally expects "|" and "fabric" as arguments
      // to perform internal piping or chaining, then we include them.
      // This was hinted at by your previous fabricArgsForPiping and runFabricCommand.
      fabricRunArgs.push("|", "fabric");

      fabricRunArgs.push("--pattern", pattern);
      fabricRunArgs.push("--model", model);

      if (context && context.trim()) {
        fabricRunArgs.push("--context", context);
      }

      console.log(`Attempting to run fabric command with args: ${fabricRunArgs.join(" ")}`);
      const command = Command.create("fabric", fabricRunArgs);
      const result = await command.execute();
      
      console.log("Fabric command stdout:", result.stdout);
      console.log("Fabric command stderr:", result.stderr);
      console.log("Fabric command exit code:", result.code);

      if (result.code === 0) {
        return result.stdout || "Fabric command executed successfully (no output).";
      } else {
        return `Error running fabric command: ${result.stderr}`;
      }
    }
  } catch (error) {
    console.error("Error running test /bin/echo command:", error);
    // It's good to see the structure of the error object too
    // console.error("Full error object:", error);
    return `Error: ${error instanceof Error ? error.message : String(error)}`;
  }
}

// JinaAI functions

export async function scrapeUrl(urlToScrape: string) {
  try {
    console.log("Scraping URL:", urlToScrape);
    const result = await Command.create("/usr/local/bin/fabric", [
      "-q",
      urlToScrape,
    ]).execute();
    console.log("Scrape command result:", result);
    // alert(`URL scraped successfully. Output: ${result.stdout}`);
  } catch (error) {
    console.error("Error scraping URL:", error);
    if (error instanceof Error) {
      alert(`Error scraping URL: ${error.message}`);
    } else {
      alert(`An unexpected error occurred: ${String(error)}`);
    }
  }
}

export async function searchQuestion(questionToSearch: string) {
  try {
    await invoke("set_is_running", { value: true });
    console.log("Searching question:", questionToSearch);
    const result = await Command.create("/usr/local/bin/fabric", [
      "-q",
      questionToSearch,
    ]).execute();
    console.log("Search question command result:", result);
    alert(`Question searched successfully. Output: ${result.stdout}`);
  } catch (error) {
    console.error("Error searching question:", error);
    if (error instanceof Error) {
      alert(`Error searching question: ${error.message}`);
    } else {
      alert(`An unexpected error occurred: ${String(error)}`);
    }
  } finally {
    await invoke("set_is_running", { value: false });
  }
}

async function runFabricCommand(
  input: string,
  flag: "-u" | "-q"
): Promise<string> {
  try {
    await invoke("set_is_running", { value: true });
    const selectedPattern = await invoke("get_secret", { key: "DEFAULT_PATTERN" });
    // TODO: Configure current pattern to be in store
    // const selectedPattern = "summarize";
    if (!selectedPattern) {
      return "Please select a pattern first.";
    }

    console.log(
      `Scraping ${flag === "-u" ? "URL" : "question"} and running pattern:`,
      input,
      selectedPattern
    );

    const result = await Command.create("/usr/local/bin/fabric", [
      flag,
      input,
      "|",
      "fabric",
      "--pattern",
      selectedPattern as string,
    ]).execute();

    console.log("Execution result:", result);
    return result.stdout;
  } catch (error) {
    console.error("Error:", error);
    return `Error: ${error instanceof Error ? error.message : String(error)}`;
  } finally {
    await invoke("set_is_running", { value: false });
  }
}

export async function scrapeUrlAndRunPattern(
  urlToScrape: string
): Promise<string> {
  return runFabricCommand(urlToScrape, "-u");
}

export async function scrapeQuestionAndRunPattern(
  questionToScrape: string
): Promise<string> {
  return runFabricCommand(questionToScrape, "-q");
}