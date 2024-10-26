import { Command } from "@tauri-apps/plugin-shell";
import { invoke } from "@tauri-apps/api/core";

export async function runPattern() {
  try {
    const selectedPattern = await invoke("get_selected_pattern");
    if (!selectedPattern) {
      alert("Please select a pattern first.");
      return;
    }

    console.log("Running pattern:", selectedPattern);
    const result = await Command.create("fabric", [
      "--pattern",
      selectedPattern as string,
    ]).execute();

    console.log("Test command result:", result);
    alert(`Command executed successfully. Output: ${result.stdout}`);
  } catch (error) {
    console.error("Error running test command:", error);
    if (error instanceof Error) {
      alert(`Error running command: ${error.message}`);
    } else {
      alert(`An unexpected error occurred: ${String(error)}`);
    }
  }
}

export async function scrapeUrl(urlToScrape: string) {
  try {
    console.log("Scraping URL:", urlToScrape);
    const result = await Command.create("fabric", [
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
    console.log("Searching question:", questionToSearch);
    const result = await Command.create("fabric", [
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
  }
}

export async function scrapeAndRunPattern(
  urlToScrape: string
): Promise<string> {
  try {
    // Step 1: Get the selected pattern
    const selectedPattern = await invoke("get_selected_pattern");
    if (!selectedPattern) {
      alert("Please select a pattern first.");
      return "Please select a pattern first.";
    }

    console.log(
      "Scraping URL and running pattern:",
      urlToScrape,
      selectedPattern
    );

    // Step 2: Create a single command that pipes the scrape result to the pattern
    const result = await Command.create("fabric", [
      "-q",
      urlToScrape,
      "|",
      "fabric",
      "--pattern",
      selectedPattern as string,
    ]).execute();

    console.log("Scrape and pattern execution result:", result);
    return `Scrape and pattern execution successful. Output: ${result.stdout}`;
  } catch (error) {
    console.error("Error in scrapeAndRunPattern:", error);
    if (error instanceof Error) {
      alert(`Error in scrapeAndRunPattern: ${error.message}`);
    } else {
      alert(`An unexpected error occurred: ${String(error)}`);
    }
  }
}
